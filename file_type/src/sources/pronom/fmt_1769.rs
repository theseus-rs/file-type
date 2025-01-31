use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1769: FileFormat = FileFormat {
    id: 2_619,
    puid: "fmt/1769",
    name: "C++ Source Code File",
    extensions: &["cpp", "cxx", "cc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
