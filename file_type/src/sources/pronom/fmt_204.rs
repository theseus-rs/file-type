use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_204: FileFormat = FileFormat {
    id: 930,
    puid: "fmt/204",
    name: "RealVideo Clip",
    extensions: &["rv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
