use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_935: FileFormat = FileFormat {
    id: 935,
    source_type: SourceType::Pronom,
    name: "Sound Designer II Audio File",
    extensions: &["sd2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
