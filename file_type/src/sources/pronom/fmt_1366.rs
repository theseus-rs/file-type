use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1366: FileFormat = FileFormat {
    id: 2_184,
    puid: "fmt/1366",
    name: "ESRI Published Map Format",
    extensions: &["pmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
