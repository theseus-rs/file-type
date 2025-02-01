use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1804: FileFormat = FileFormat {
    id: 2_655,
    puid: "fmt/1804",
    name: "B Source Code File",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
