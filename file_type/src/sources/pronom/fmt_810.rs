use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_810: FileFormat = FileFormat {
    id: 1_610,
    puid: "fmt/810",
    name: "StarOffice Draw",
    extensions: &["sdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
