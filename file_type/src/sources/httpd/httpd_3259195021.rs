use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3259195021: FileFormat = FileFormat {
    id: 3_259_195_021,
    source_type: SourceType::Httpd,
    name: "dna",
    extensions: &["dna"],
    media_types: &["application/vnd.dna"],
    signatures: &[],
    related_formats: &[],
};
