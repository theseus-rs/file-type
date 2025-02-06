use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1721: FileFormat = FileFormat {
    id: 1_721,
    source_type: SourceType::Pronom,
    name: "ESRI ArcMap Document",
    extensions: &["mxd", "mxt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
