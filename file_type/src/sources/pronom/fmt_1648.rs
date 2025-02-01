use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1648: FileFormat = FileFormat {
    id: 2_475,
    puid: "fmt/1648",
    name: "Crystal Reports File",
    extensions: &["rpt"],
    media_types: &["application/x-rpt"],
    internal_signatures: &[],
    related_formats: &[],
};
