use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_503: FileFormat = FileFormat {
    id: 503,
    source_type: SourceType::Pronom,
    name: "Lotus Notes File",
    extensions: &["box"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
