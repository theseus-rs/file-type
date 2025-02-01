use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1797: FileFormat = FileFormat {
    id: 2_648,
    puid: "fmt/1797",
    name: "SHA512 File",
    extensions: &["sha512"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
