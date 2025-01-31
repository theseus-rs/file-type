use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1229: FileFormat = FileFormat {
    id: 2_041,
    puid: "fmt/1229",
    name: "Sibelius Sound Set Definition",
    extensions: &["set"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
