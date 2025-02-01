use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_811: FileFormat = FileFormat {
    id: 1_611,
    puid: "fmt/811",
    name: "StarOffice Draw",
    extensions: &["sdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
