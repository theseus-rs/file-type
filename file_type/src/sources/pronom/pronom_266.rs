use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_266: FileFormat = FileFormat {
    id: 266,
    source_type: SourceType::Pronom,
    name: "Unisys (Sperry) System Data File",
    extensions: &["sdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
