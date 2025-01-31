use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1149: FileFormat = FileFormat {
    id: 1_959,
    puid: "fmt/1149",
    name: "Markdown",
    extensions: &["md", "markdown"],
    media_types: &["text/markdown"],
    internal_signatures: &[],
    related_formats: &[],
};
