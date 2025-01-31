use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1847: FileFormat = FileFormat {
    id: 2_699,
    puid: "fmt/1847",
    name: "Esri ArcMap Label file",
    extensions: &["lxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
