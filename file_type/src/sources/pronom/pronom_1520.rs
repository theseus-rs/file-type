use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1520: FileFormat = FileFormat {
    id: 1_520,
    source_type: SourceType::Pronom,
    name: "VLW Font File",
    extensions: &["vlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
