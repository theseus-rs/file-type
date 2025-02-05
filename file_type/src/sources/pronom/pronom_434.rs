use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_434: FileFormat = FileFormat {
    id: 434,
    source_type: SourceType::Pronom,
    name: "8-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
