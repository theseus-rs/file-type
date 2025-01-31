use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1959: FileFormat = FileFormat {
    id: 2_824,
    puid: "fmt/1959",
    name: "Melco OFM Project",
    extensions: &["ofm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
