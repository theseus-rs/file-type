use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_317: FileFormat = FileFormat {
    id: 317,
    source_type: SourceType::Pronom,
    name: "ESRI MapInfo Data File",
    extensions: &["mid"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
