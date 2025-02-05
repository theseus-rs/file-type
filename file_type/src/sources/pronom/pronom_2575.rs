use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2575: FileFormat = FileFormat {
    id: 2_575,
    source_type: SourceType::Pronom,
    name: "Data File",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
