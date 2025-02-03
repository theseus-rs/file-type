use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2699: FileFormat = FileFormat {
    id: 2_699,
    source_type: SourceType::Pronom,
    name: "Esri ArcMap Label file",
    extensions: &["lxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
