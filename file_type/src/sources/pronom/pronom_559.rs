use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_559: FileFormat = FileFormat {
    id: 559,
    source_type: SourceType::Pronom,
    name: "Dia Graphics Format",
    extensions: &["dia"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
