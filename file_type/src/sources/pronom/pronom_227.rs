use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_227: FileFormat = FileFormat {
    id: 227,
    source_type: SourceType::Pronom,
    name: "Java Servlet Page",
    extensions: &["jsp"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
