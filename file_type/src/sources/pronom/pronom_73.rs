use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_73: FileFormat = FileFormat {
    id: 73,
    source_type: SourceType::Pronom,
    name: "Wordperfect Secondary File",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
