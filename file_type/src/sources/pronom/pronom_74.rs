use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_74: FileFormat = FileFormat {
    id: 74,
    source_type: SourceType::Pronom,
    name: "Wordperfect Secondary File",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
