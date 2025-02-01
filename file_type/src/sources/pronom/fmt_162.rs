use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_162: FileFormat = FileFormat {
    id: 889,
    puid: "fmt/162",
    name: "Microsoft Multiplan",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
