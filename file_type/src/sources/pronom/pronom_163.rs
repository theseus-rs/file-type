use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_163: FileFormat = FileFormat {
    id: 163,
    source_type: SourceType::Pronom,
    name: "Plain Text File",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
