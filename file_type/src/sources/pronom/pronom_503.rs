use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_503: FileFormat = FileFormat {
    id: 503,
    source_type: SourceType::Pronom,
    name: "Lotus Notes File",
    extensions: &["box"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
