use log::{debug, error, info, log_enabled, Level};
use std::io;
use std::io::Read;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

mod contexts;
pub use contexts::*;

use env_logger::Builder;
use log::LevelFilter;

use polars::frame::DataFrame;

fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut markdown_input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut markdown_input)?;

    let parser = Parser::new_ext(&markdown_input, Options::all());

    let mut contexts = Contexts::new();
    let events_in_contexts = parser
        .into_iter()
        .map(|event| match event {
            Event::Start(ref tag) => {
                contexts.enter_context(tag.clone());
                (contexts.clone(), event)
            }
            Event::End(ref tag) => {
                contexts.quit_context(tag.clone());
                (contexts.clone(), event)
            }
            _ => (contexts.clone(), event),
        })
        .collect::<Vec<(Contexts, Event)>>();

    let mut tables = events_in_contexts
        .iter()
        .filter(|(context, _event)| {
            context
                .contexts()
                .iter()
                .any(|e| matches!(e, Tag::Table(_)))
        })
        .map(|(_, event)| event)
        .collect::<Vec<&Event>>();

    // il faudrait split sur start table ou table end ! (s'il ya plusieures tables dans le fichier)

    let x = tables
        .split(|elem| {
            matches!(elem, Event::Start(Tag::Table(_)))
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|e| {
            let mut header: Vec<String> = vec![];

            e.iter().filter(||)
            headers
        })
        .collect::<Vec<Vec<String>>>();

    println!("{:?}", x);

    // for t in tables {
    //     let mut df = DataFrame::default();

    //     println!("T {:?}", t);

    // }

    let has_table = events_in_contexts.iter().any(|(context, _event)| {
        context
            .contexts()
            .iter()
            .any(|e| matches!(e, Tag::Table(_)))
    });

    let h2 = events_in_contexts.iter().any(|(context, event)| {
        context
            .contexts()
            .iter()
            .any(|e| matches!(e, Tag::Heading(HeadingLevel::H2, _, _)))
            && event == &Event::Text("Définition".into())
    });

    if has_table {
        info!("has table {:?}", has_table);
    } else {
        error!("has table {:?}", has_table);
    }

    if h2 {
        info!("has définition {:?}", h2);
    } else {
        error!("has définition {:?}", h2);
    }

    Ok(())
}
