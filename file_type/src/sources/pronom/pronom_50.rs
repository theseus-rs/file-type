use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_50: FileFormat = FileFormat {
    id: 50,
    source_type: SourceType::Pronom,
    name: "7-bit ANSI Text",
    extensions: &["ans"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
