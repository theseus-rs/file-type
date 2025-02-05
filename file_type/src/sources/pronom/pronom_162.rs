use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_162: FileFormat = FileFormat {
    id: 162,
    source_type: SourceType::Pronom,
    name: "Fixed Width Values Text File",
    extensions: &[],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
