use quick_xml::Reader;
use quick_xml::events::Event;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Reads plain text, .pdf, or .docx files and returns the extracted content as a String
pub fn read_file_content(path_str: &str) -> Result<String, String> {
    let path = Path::new(path_str);
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("");

    match extension.to_lowercase().as_str() {
        "pdf" => extract_pdf_text(path_str),
        "docx" => extract_docx_text(path_str),
        _ => read_plain_text(path_str), // Fallback for .txt, .rs, .md, etc.
    }
}

// Handler 1: Standard Plain Text Files
fn read_plain_text(path_str: &str) -> Result<String, String> {
    std::fs::read_to_string(path_str)
        .map_err(|err| format!("Failed to read text file: {err}"))
}

// Handler 2: Extracting Text from PDF
fn extract_pdf_text(path_str: &str) -> Result<String, String> {
    pdf_extract::extract_text(path_str)
        .map_err(|err| format!("Failed to extract PDF text: {err}"))
}

// Handler 3: Extracting Text from Word (.docx)
fn extract_docx_text(path_str: &str) -> Result<String, String> {
    let file = File::open(path_str).map_err(|err| format!("Could not open file: {err}"))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|err| format!("Failed to open DOCX archive: {err}"))?;

    let mut xml_file = archive
        .by_name("word/document.xml")
        .map_err(|err| format!("DOCX is missing word/document.xml: {err}"))?;

    let mut xml_bytes = Vec::new();
    xml_file
        .read_to_end(&mut xml_bytes)
        .map_err(|err| format!("Failed to read DOCX XML content: {err}"))?;

    let mut reader = Reader::from_reader(xml_bytes.as_slice());
    reader.config_mut().trim_text(true);

    let mut extracted_text = String::new();
    let mut in_text = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"t" {
                    in_text = true;
                }
            }
            Ok(Event::Text(e)) => {
                if in_text {
                    let text_bytes = e.into_owned();
                    let text = String::from_utf8_lossy(&text_bytes);
                    extracted_text.push_str(&text);
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"t" {
                    in_text = false;
                } else if e.name().as_ref() == b"p" {
                    extracted_text.push('\n');
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to parse DOCX XML: {err}")),
        }
        buf.clear();
    }

    Ok(extracted_text)
}