use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2692: FileFormat = FileFormat {
    id: 2_692,
    source_type: SourceType::Pronom,
    name: "WACZ",
    extensions: &["wacz"],
    media_types: &["application/x-wacz"],
    signatures: &[],
    related_formats: &[],
};
