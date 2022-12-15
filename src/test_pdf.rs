use std::{path::{self, PathBuf}, io::BufWriter, fs::File};
use pdf_extract::*;

pub fn main() {
    let fp = "/home/mkd/Documents/Julien DURIS_ Resume.pdf";
    let path = path::Path::new(fp);
    let mut output_buf = PathBuf::new();
    output_buf.push(path.file_name().unwrap());
    output_buf.set_extension("html");
    let mut output_file = BufWriter::new(File::create(output_buf).unwrap());
    let doc = Document::load(path).unwrap();
    print_metadata(&doc);

    let mut output: Box<dyn OutputDev> = Box::new(HTMLOutput::new(&mut output_file));

    output_doc(&doc, output.as_mut()).unwrap();
}