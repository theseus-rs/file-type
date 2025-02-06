use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_398048020: FileFormat = FileFormat {
    id: 398_048_020,
    source_type: SourceType::Iana,
    name: "vnd.pocketlearn",
    extensions: &[],
    media_types: &["application/vnd.pocketlearn"],
    signatures: &[],
    related_formats: &[],
};
