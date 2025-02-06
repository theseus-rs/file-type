use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_51: FileFormat = FileFormat {
    id: 51,
    source_type: SourceType::Pronom,
    name: "7-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
