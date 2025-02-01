use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1652: FileFormat = FileFormat {
    id: 2_479,
    puid: "fmt/1652",
    name: "Typescript",
    extensions: &["ts", "tsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
