use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_163: FileFormat = FileFormat {
    id: 163,
    source_type: SourceType::Pronom,
    name: "Plain Text File",
    extensions: &["txt"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
