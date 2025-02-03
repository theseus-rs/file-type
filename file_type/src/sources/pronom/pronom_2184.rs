use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2184: FileFormat = FileFormat {
    id: 2_184,
    source_type: SourceType::Pronom,
    name: "ESRI Published Map Format",
    extensions: &["pmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
