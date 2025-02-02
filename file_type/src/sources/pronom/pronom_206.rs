use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_206: FileFormat = FileFormat {
    id: 206,
    source_type: SourceType::Pronom,
    name: "Stats+ Data File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
