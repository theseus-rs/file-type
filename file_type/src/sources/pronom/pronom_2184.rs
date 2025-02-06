use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2184: FileFormat = FileFormat {
    id: 2_184,
    source_type: SourceType::Pronom,
    name: "ESRI Published Map Format",
    extensions: &["pmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
