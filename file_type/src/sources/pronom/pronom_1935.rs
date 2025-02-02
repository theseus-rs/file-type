use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1935: FileFormat = FileFormat {
    id: 1_935,
    source_type: SourceType::Pronom,
    name: "JASCO JWS Format",
    extensions: &["jws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
