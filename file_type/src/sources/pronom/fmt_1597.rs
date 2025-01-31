use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1597: FileFormat = FileFormat {
    id: 2_424,
    puid: "fmt/1597",
    name: "PageMaker Template File",
    extensions: &["pt5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
