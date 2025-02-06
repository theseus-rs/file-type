use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1794: FileFormat = FileFormat {
    id: 1_794,
    source_type: SourceType::Pronom,
    name: "ESRI ArcGlobe Document",
    extensions: &["3dd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
