use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1610: FileFormat = FileFormat {
    id: 1_610,
    source_type: SourceType::Pronom,
    name: "StarOffice Draw",
    extensions: &["sdd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
