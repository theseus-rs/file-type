use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1721: FileFormat = FileFormat {
    id: 1_721,
    source_type: SourceType::Pronom,
    name: "ESRI ArcMap Document",
    extensions: &["mxd", "mxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
