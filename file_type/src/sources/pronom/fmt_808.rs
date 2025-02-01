use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_808: FileFormat = FileFormat {
    id: 1_608,
    puid: "fmt/808",
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
