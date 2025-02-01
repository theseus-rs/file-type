use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_992: FileFormat = FileFormat {
    id: 1_797,
    puid: "fmt/992",
    name: "SHA1 File",
    extensions: &["sha1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
