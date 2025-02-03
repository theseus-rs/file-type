use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_434: FileFormat = FileFormat {
    id: 434,
    source_type: SourceType::Pronom,
    name: "8-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
