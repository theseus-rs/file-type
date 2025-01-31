use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_160: FileFormat = FileFormat {
    id: 227,
    puid: "x-fmt/160",
    name: "Java Servlet Page",
    extensions: &["jsp"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
