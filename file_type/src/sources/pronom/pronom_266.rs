use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_266: FileFormat = FileFormat {
    id: 266,
    source_type: SourceType::Pronom,
    name: "Unisys (Sperry) System Data File",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
