use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17072281593186703027: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pocketlearn",
    extensions: &["plf"],
    media_types: &["application/vnd.pocketlearn"],
    internal_signatures: &[],
    related_formats: &[],
};
