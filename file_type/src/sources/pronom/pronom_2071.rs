use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2071: FileFormat = FileFormat {
    id: 2_071,
    source_type: SourceType::Pronom,
    name: "ESRI Code Page File",
    extensions: &["cpg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
