use std::io;
use std::io::Read;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

#[derive(Debug, Clone, PartialEq)]
pub struct Contexts<'a>(Vec<Tag<'a>>);

impl<'a> Contexts<'a> {
    fn new() -> Self {
        Contexts(vec![])
    }

    fn push_tag(&mut self, tag: Tag<'a>) {
        self.0.push(tag);
    }

    fn pop_tag(&mut self, tag: Tag<'a>) {
        if let Some(pos) = self.0.iter().rposition(|elem| elem == &tag) {
            self.0.remove(pos);
        }
    }
}

fn main() -> io::Result<()> {
    let mut markdown_input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut markdown_input)?;

    let parser = Parser::new_ext(&markdown_input, Options::all());

    let mut contexts = Contexts::new();
    let events_in_contexts = parser
        .into_iter()
        .map(|event| match event {
            Event::Start(ref tag) => {
                contexts.push_tag(tag.clone());
                (contexts.clone(), event)
            }
            Event::End(ref tag) => {
                contexts.pop_tag(tag.clone());
                (contexts.clone(), event)
            }
            _ => (contexts.clone(), event),
        })
        .collect::<Vec<(Contexts, Event)>>();

    let tables = events_in_contexts
        .iter()
        .any(|(context, _event)| context.0.iter().any(|e| matches!(e, Tag::Table(_))));

    let h2 = events_in_contexts
        .iter()
        .any(|(context, event)| {
            context
                .0
                .iter()
                .any(|e| matches!(e, Tag::Heading(HeadingLevel::H2, _, _)))
                && event == &Event::Text("Définition".into())
        });

    println!("has table {:?}", tables);
    println!("has définition {:?}", h2);

    Ok(())
}
