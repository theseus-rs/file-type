use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_993: FileFormat = FileFormat {
    id: 1_798,
    puid: "fmt/993",
    name: "MD5 File",
    extensions: &["md5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
