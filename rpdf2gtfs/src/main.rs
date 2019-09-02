extern crate pdf;
extern crate pdf_extract;
extern crate lopdf;

use std::env::args;
use std::time::SystemTime;
use std::fs;
use std::io::Write;

use pdf::file::File;
use pdf::print_err;
use pdf::object::*;
use pdf::content::*;
use pdf::primitive::Primitive;

fn add_primitive(p: &Primitive, out: &mut String) {
    // println!("p: {:?}", p);
    match p {
        &Primitive::String(ref s) => if let Ok(text) = s.as_str() {
            out.push_str(text);
        }
        &Primitive::Array(ref a) => for p in a.iter() {
            add_primitive(p, out);
        }
        _ => ()
    }
}

fn pdf() {
    let path = args().nth(1).expect("no file given");
    println!("read: {}", path);
    let now = SystemTime::now();
    let file = File::<Vec<u8>>::open(&path).unwrap_or_else(|e| print_err(e));

    // let fonts: Vec<_> =  file.pages()
    //     .filter_map(|page| page.resources.as_ref())
    //     .filter_map(|res| res.fonts.as_ref())
    //     // .flat_map(|xo| xo)
    //     // .filter_map(|(_, o)| Some(o) )
    //     .collect();
    // println!("Fonts {:?}", fonts);
    // let xobjects: Vec<_> =  file.pages()
    //     .filter_map(|page| page.resources.as_ref())
    //     .filter_map(|res| res.xobjects.as_ref())
    //     .collect();
    // println!("XObjs {:?}", xobjects);
    // let ext_g_state: Vec<_> =  file.pages()
    //     .filter_map(|page| page.resources.as_ref())
    //     .filter_map(|res| res.ext_g_state.as_ref())
    //     .collect();
    // println!("ext_g_state {:?}", ext_g_state);

    let mut out = String::new();
    for page in file.pages() {
        for content in &page.contents {
            for &Operation { ref operator, ref operands } in &content.operations {
                println!("{} {:?}", operator, operands);
                match operator.as_str() {
                    "Tj" | "TJ" | "BT" => operands.iter().for_each(|p| add_primitive(p, &mut out)),
                    _ => {}
                }
            }
        }
    }
    println!("{}", out);
}


// fn myoutput(doc: &Document) {
//     let empty_resources = &Dictionary::new();

//     let pages = doc.get_pages();
//     let mut p = Processor::new();
//     for dict in pages {
//         let page_num = dict.0;
//         let page_dict = doc.get_object(dict.1).unwrap().as_dict().unwrap();
//         dlog!("page {} {:?}", page_num, page_dict);
//         // XXX: Some pdfs lack a Resources directory
//         let resources = get_inherited(doc, page_dict, b"Resources").unwrap_or(empty_resources);
//         dlog!("resources {:?}", resources);

//         // pdfium searches up the page tree for MediaBoxes as needed
//         let media_box: Vec<f64> = get_inherited(doc, page_dict, b"MediaBox").expect("MediaBox");
//         let media_box = MediaBox { llx: media_box[0], lly: media_box[1], urx: media_box[2], ury: media_box[3] };

//         let art_box = get::<Option<Vec<f64>>>(&doc, page_dict, b"ArtBox")
//             .map(|x| (x[0], x[1], x[2], x[3]));

//         output.begin_page(page_num, &media_box, art_box);

//         p.process_stream(&doc, doc.get_page_content(dict.1).unwrap(), resources,&media_box, output, page_num);

//         output.end_page();
//     }
// }

fn pdfextract() {

    use std::env;
    use std::path::PathBuf;
    use std::path;
    use std::io::BufWriter;
    use std::fs::File;
    use pdf_extract::*;
    use lopdf::*;
    //let output_kind = "html";
    //let output_kind = "txt";
    //let output_kind = "svg";
    let file = env::args().nth(1).unwrap();
    let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());
    println!("{}", file);
    let path = path::Path::new(&file);
    let filename = path.file_name().expect("expected a filename");
    let mut output_file = PathBuf::new();
    output_file.push(filename);
    output_file.set_extension(&output_kind);
    let mut output_file = BufWriter::new(File::create(output_file).expect("could not create output"));
    let doc = Document::load(path).unwrap();

    print_metadata(&doc);

    let mut output: Box<OutputDev> = match output_kind.as_ref() {
        "txt" => Box::new(PlainTextOutput::new(&mut output_file as (&mut std::io::Write))),
        "html" => Box::new(HTMLOutput::new(&mut output_file)),
        "svg" => Box::new(SVGOutput::new(&mut output_file)),
        _ => panic!(),
    };

    output_doc(&doc, output.as_mut());
}

fn main() {
    // pdf();
    pdfextract();
}
