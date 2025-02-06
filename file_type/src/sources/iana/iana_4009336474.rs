use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4009336474: FileFormat = FileFormat {
    id: 4_009_336_474,
    source_type: SourceType::Iana,
    name: "DVI4",
    extensions: &[],
    media_types: &["audio/DVI4"],
    signatures: &[],
    related_formats: &[],
};
