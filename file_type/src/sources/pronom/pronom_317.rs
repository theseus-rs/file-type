use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_317: FileFormat = FileFormat {
    id: 317,
    source_type: SourceType::Pronom,
    name: "ESRI MapInfo Data File",
    extensions: &["mid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
