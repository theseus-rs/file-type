use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11841558493466156020: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sgml",
    extensions: &["sgml", "sgm"],
    media_types: &["text/sgml"],
    internal_signatures: &[],
    related_formats: &[],
};
