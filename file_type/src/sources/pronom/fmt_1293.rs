use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1293: FileFormat = FileFormat {
    id: 2_111,
    puid: "fmt/1293",
    name: "602Text Document",
    extensions: &["wpd", "wpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
