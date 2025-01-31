use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_667: FileFormat = FileFormat {
    id: 1_466,
    puid: "fmt/667",
    name: "Photoshop Curve File",
    extensions: &["acv", "atf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
