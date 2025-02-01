use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1125: FileFormat = FileFormat {
    id: 1_935,
    puid: "fmt/1125",
    name: "JASCO JWS Format",
    extensions: &["jws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
