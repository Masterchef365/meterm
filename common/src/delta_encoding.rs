use egui::{
    epaint::{ClippedShape, TextShape},
    text::LayoutJob,
    Context, FullOutput, Galley, Shape,
};

use std::{collections::HashMap, sync::Arc};

use crate::hash_abuse::{EqByHash, HashBySerialize};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum UpdateData {
    FullUpdate(FullOutput),
    /// Used to build a FullOutput on the other side
    Partial(FullOutput, Vec<PartialUpdate>),
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum PartialUpdate {
    Reference(usize),
    Shape(ClippedShape),
}

pub struct Encoder {
    memory: HashMap<EqByHash<HashBySerialize<ClippedShape>>, usize>,
    pub interval: usize,
    counter: usize,
}

pub struct Decoder {
    galley_cache: HashMap<EqByHash<HashBySerialize<Arc<LayoutJob>>>, Arc<Galley>>,
    memory: Option<FullOutput>,
    pub debug_mode: bool,
}

impl Default for Encoder {
    fn default() -> Self {
        Self {
            memory: Default::default(),
            interval: 90,
            counter: 0,
        }
    }
}

impl Encoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn encode(&mut self, data: &FullOutput) -> UpdateData {
        let mut do_partial_update = !self.memory.is_empty();
        self.counter += 1;
        if self.counter > self.interval {
            do_partial_update = false;
            self.counter = 0;
        }

        self.encode_manual_partial(data, do_partial_update)
    }

    pub fn encode_manual_partial(&mut self, data: &FullOutput, partial: bool) -> UpdateData {
        if partial {
            let mut data = data.clone();
            let partial_updates = data
                .shapes
                .drain(..)
                .map(|shape| {
                    let casted = EqByHash(HashBySerialize(shape));
                    if let Some(&index) = self.memory.get(&casted) {
                        PartialUpdate::Reference(index)
                    } else {
                        PartialUpdate::Shape(casted.0 .0)
                    }
                })
                .collect();

            UpdateData::Partial(data, partial_updates)
        } else {
            self.memory.clear();

            for (idx, shape) in data.shapes.iter().enumerate() {
                self.memory
                    .insert(EqByHash(HashBySerialize(shape.clone())), idx);
            }

            UpdateData::FullUpdate(data.clone())
        }
    }
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            memory: None,
            debug_mode: false,
            galley_cache: Default::default(),
        }
    }

    pub fn decode(&mut self, update: UpdateData, ctx: &Context) -> Option<FullOutput> {
        match update {
            UpdateData::FullUpdate(mut full) => {
                let doctored = full
                    .shapes
                    .drain(..)
                    .map(|shape| self.doctor_shape(ctx, shape))
                    .collect();
                full.shapes = doctored;

                self.galley_cache = Default::default();
                self.memory = Some(full.clone());
                Some(full)
            }
            UpdateData::Partial(mut upd, partials) => {
                for part in partials {
                    match part {
                        PartialUpdate::Shape(shape) => {
                            upd.shapes.push(self.doctor_shape(ctx, shape))
                        }
                        PartialUpdate::Reference(index) => {
                            if !self.debug_mode {
                                upd.shapes.push(self.memory.as_mut()?.shapes[index].clone());
                            }
                        }
                    };
                }
                Some(upd)
            }
        }
    }

    pub fn doctor_shape(&mut self, ctx: &Context, mut shape: ClippedShape) -> ClippedShape {
        match &mut shape.shape {
            Shape::Text(text) => self.doctor_text(ctx, text),
            _ => (),
        }

        shape
    }

    pub fn doctor_text(&mut self, ctx: &Context, shape: &mut TextShape) {
        let job = shape.galley.job.clone();
        let galley = self
            .galley_cache
            .entry(EqByHash(HashBySerialize(job.clone())))
            .or_insert_with(|| ctx.fonts(|r| r.layout_job(Arc::unwrap_or_clone(job))))
            .clone();

        shape.galley = galley;
    }
}
