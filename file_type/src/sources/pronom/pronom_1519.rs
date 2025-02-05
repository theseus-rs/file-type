use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1519: FileFormat = FileFormat {
    id: 1_519,
    source_type: SourceType::Pronom,
    name: "MBOX",
    extensions: &["mbox"],
    media_types: &["application/mbox"],
    signatures: &[],
    related_formats: &[],
};
