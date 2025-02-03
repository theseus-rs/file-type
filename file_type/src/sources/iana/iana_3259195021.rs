use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3259195021: FileFormat = FileFormat {
    id: 3_259_195_021,
    source_type: SourceType::Iana,
    name: "vnd.dna",
    extensions: &[],
    media_types: &["application/vnd.dna"],
    internal_signatures: &[],
    related_formats: &[],
};
