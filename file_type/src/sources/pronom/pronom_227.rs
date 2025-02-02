use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_227: FileFormat = FileFormat {
    id: 227,
    source_type: SourceType::Pronom,
    name: "Java Servlet Page",
    extensions: &["jsp"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
