use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1794: FileFormat = FileFormat {
    id: 1_794,
    source_type: SourceType::Pronom,
    name: "ESRI ArcGlobe Document",
    extensions: &["3dd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
