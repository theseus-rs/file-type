use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1959: FileFormat = FileFormat {
    id: 1_959,
    source_type: SourceType::Pronom,
    name: "Markdown",
    extensions: &["md", "markdown"],
    media_types: &["text/markdown"],
    signatures: &[],
    related_formats: &[],
};
