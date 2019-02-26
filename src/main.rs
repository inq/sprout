#[macro_use]
extern crate failure;

mod page;

use lopdf::{Document, Object};
use page::Page;

fn main() -> Result<(), failure::Error> {
    let mut doc = Document::load("target/input.pdf")?;
    let pages = doc.get_pages();
    for (_page, page_id) in pages {
        for object_id in doc.get_page_contents(page_id) {
            if let Some(Object::Stream(ref mut stream)) = doc.get_object_mut(object_id) {
                let mut page = Page::new();
                page.read_stream(stream)?;
            }
        }
        break; // TODO: Multiple pages
    }
    doc.save("target/output.pdf")?;
    Ok(())
}
