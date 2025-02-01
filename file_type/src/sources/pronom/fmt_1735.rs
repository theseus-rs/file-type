use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1735: FileFormat = FileFormat {
    id: 2_580,
    puid: "fmt/1735",
    name: "C/C++ Header File",
    extensions: &["h", "hpp", "hxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
