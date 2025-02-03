use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1945973476: FileFormat = FileFormat {
    id: 1_945_973_476,
    source_type: SourceType::Httpd,
    name: "cif",
    extensions: &["cif"],
    media_types: &["chemical/x-cif"],
    internal_signatures: &[],
    related_formats: &[],
};
