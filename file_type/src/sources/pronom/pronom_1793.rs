use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1793: FileFormat = FileFormat {
    id: 1_793,
    source_type: SourceType::Pronom,
    name: "ESRI ArcScene Document",
    extensions: &["sxd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
