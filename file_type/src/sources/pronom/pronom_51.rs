use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_51: FileFormat = FileFormat {
    id: 51,
    source_type: SourceType::Pronom,
    name: "7-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
