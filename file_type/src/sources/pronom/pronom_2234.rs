use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2234: FileFormat = FileFormat {
    id: 2_234,
    source_type: SourceType::Pronom,
    name: "GST Publisher File",
    extensions: &["dtp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
