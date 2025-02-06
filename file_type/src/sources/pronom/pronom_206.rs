use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_206: FileFormat = FileFormat {
    id: 206,
    source_type: SourceType::Pronom,
    name: "Stats+ Data File",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
