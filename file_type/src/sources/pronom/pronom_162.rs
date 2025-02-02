use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_162: FileFormat = FileFormat {
    id: 162,
    source_type: SourceType::Pronom,
    name: "Fixed Width Values Text File",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
