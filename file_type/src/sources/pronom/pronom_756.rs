use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_756: FileFormat = FileFormat {
    id: 756,
    source_type: SourceType::Pronom,
    name: "StarOffice Draw",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
