use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_989: FileFormat = FileFormat {
    id: 1_794,
    puid: "fmt/989",
    name: "ESRI ArcGlobe Document",
    extensions: &["3dd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
