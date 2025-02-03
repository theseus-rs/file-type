use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_5639650: FileFormat = FileFormat {
    id: 5_639_650,
    source_type: SourceType::Iana,
    name: "vnd.pco.b16",
    extensions: &[],
    media_types: &["image/vnd.pco.b16"],
    internal_signatures: &[],
    related_formats: &[],
};
