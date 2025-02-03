use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_649945256: FileFormat = FileFormat {
    id: 649_945_256,
    source_type: SourceType::Httpd,
    name: "frogans ltf",
    extensions: &["ltf"],
    media_types: &["application/vnd.frogans.ltf"],
    internal_signatures: &[],
    related_formats: &[],
};
