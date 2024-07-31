use meterm_common::egui::{Context, Id};

#[derive(Default)]
pub struct ClientIdentifier {
    next_idx: usize,
}

impl ClientIdentifier {
    pub fn get_idx(&mut self, ctx: &Context) -> usize {
        ctx.data_mut(|writer| {

            let id = Id::new("_client_identifier_util");

            *writer.get_temp_mut_or_insert_with(id, || {
                let ret = self.next_idx;
                self.next_idx += 1;
                ret
            })
        })
    }
}
