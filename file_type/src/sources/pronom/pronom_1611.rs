use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1611: FileFormat = FileFormat {
    id: 1_611,
    source_type: SourceType::Pronom,
    name: "StarOffice Draw",
    extensions: &["sdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
