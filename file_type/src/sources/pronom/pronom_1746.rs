use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1746: FileFormat = FileFormat {
    id: 1_746,
    source_type: SourceType::Pronom,
    name: "Back Up File",
    extensions: &["bak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
