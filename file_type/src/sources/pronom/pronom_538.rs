use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_538: FileFormat = FileFormat {
    id: 538,
    source_type: SourceType::Pronom,
    name: "VisiCalc Database",
    extensions: &["dif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
