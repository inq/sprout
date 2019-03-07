#[macro_use]
extern crate failure;

mod common;
mod parser;
mod recognizer;
mod smf;
mod svg;

use lopdf::{Document, Object};
use parser::Parser;
use recognizer::Recognizer;

fn main() -> Result<(), failure::Error> {
    let mut doc = Document::load("target/input.pdf")?;
    let pages = doc.get_pages();
    for (_page, page_id) in pages {
        for object_id in doc.get_page_contents(page_id) {
            println!("{:?}", page_id);
            if let (21, 0) = page_id {
                if let Some(Object::Stream(ref mut stream)) = doc.get_object_mut(object_id) {
                    let parsed = Parser::new(stream)?;
                    let mut recognized = Recognizer::new(parsed);
                    recognized.process();
                }
            }
        }
        //break; // TODO: Multiple pages
    }
    doc.save("target/output.pdf")?;
    Ok(())
}
