use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1422: FileFormat = FileFormat {
    id: 2_240,
    puid: "fmt/1422",
    name: "Corel Photo House Image",
    extensions: &["cps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
