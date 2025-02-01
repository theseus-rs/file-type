use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_877: FileFormat = FileFormat {
    id: 1_681,
    puid: "fmt/877",
    name: "Corel Presentation",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
