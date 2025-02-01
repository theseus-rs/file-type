use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_812: FileFormat = FileFormat {
    id: 1_612,
    puid: "fmt/812",
    name: "StarOffice Writer",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
