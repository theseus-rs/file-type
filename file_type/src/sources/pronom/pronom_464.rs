use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_464: FileFormat = FileFormat {
    id: 464,
    source_type: SourceType::Pronom,
    name: "AutoSketch Drawing",
    extensions: &["skf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
