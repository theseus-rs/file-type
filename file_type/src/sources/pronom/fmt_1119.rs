use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1119: FileFormat = FileFormat {
    id: 1_929,
    puid: "fmt/1119",
    name: "Jupyter Python Notebook",
    extensions: &["ipynb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
