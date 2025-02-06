use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_350150928: FileFormat = FileFormat {
    id: 350_150_928,
    source_type: SourceType::Httpd,
    name: "xspf xml",
    extensions: &["xspf"],
    media_types: &["application/xspf+xml"],
    signatures: &[],
    related_formats: &[],
};
