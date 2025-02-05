use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_433: FileFormat = FileFormat {
    id: 433,
    source_type: SourceType::Pronom,
    name: "8-bit ANSI Text",
    extensions: &["ans"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
