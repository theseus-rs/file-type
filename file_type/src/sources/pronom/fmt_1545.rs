use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1545: FileFormat = FileFormat {
    id: 2_370,
    puid: "fmt/1545",
    name: "NeoDesk Icon File",
    extensions: &["nic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
