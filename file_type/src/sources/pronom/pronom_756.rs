use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_756: FileFormat = FileFormat {
    id: 756,
    source_type: SourceType::Pronom,
    name: "StarOffice Draw",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
