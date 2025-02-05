use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1746: FileFormat = FileFormat {
    id: 1_746,
    source_type: SourceType::Pronom,
    name: "Back Up File",
    extensions: &["bak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
