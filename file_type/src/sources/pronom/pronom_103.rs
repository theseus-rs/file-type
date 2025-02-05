use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_103: FileFormat = FileFormat {
    id: 103,
    source_type: SourceType::Pronom,
    name: "Log File",
    extensions: &["log"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
