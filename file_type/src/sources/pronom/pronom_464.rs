use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_464: FileFormat = FileFormat {
    id: 464,
    source_type: SourceType::Pronom,
    name: "AutoSketch Drawing",
    extensions: &["skf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
