use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_41: FileFormat = FileFormat {
    id: 41,
    source_type: SourceType::Pronom,
    name: "Macintosh Text File",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
