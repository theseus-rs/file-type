use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6150027229732099657: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "doom",
    extensions: &["wad"],
    media_types: &["application/x-doom"],
    internal_signatures: &[],
    related_formats: &[],
};
