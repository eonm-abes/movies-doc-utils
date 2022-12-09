use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

#[derive(Debug, Clone, PartialEq)]
pub struct Contexts<'a>(Vec<Tag<'a>>);

impl<'a> Contexts<'a> {
    pub fn new() -> Self {
        Contexts(vec![])
    }

    pub fn enter_context(&mut self, tag: Tag<'a>) {
        self.0.push(tag);
    }

    pub fn quit_context(&mut self, tag: Tag<'a>) {
        if let Some(pos) = self.0.iter().rposition(|elem| elem == &tag) {
            self.0.remove(pos);
        }
    }

    pub fn contexts(&self) -> &Vec<Tag<'a>> {
        &self.0
    }
}

