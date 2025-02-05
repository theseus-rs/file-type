use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1898: FileFormat = FileFormat {
    id: 1_898,
    source_type: SourceType::Pronom,
    name: "Exclude File",
    extensions: &["exclude"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
