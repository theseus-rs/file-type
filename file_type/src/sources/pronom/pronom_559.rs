use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_559: FileFormat = FileFormat {
    id: 559,
    source_type: SourceType::Pronom,
    name: "Dia Graphics Format",
    extensions: &["dia"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
