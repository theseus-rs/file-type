use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_103: FileFormat = FileFormat {
    id: 103,
    source_type: SourceType::Pronom,
    name: "Log File",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
