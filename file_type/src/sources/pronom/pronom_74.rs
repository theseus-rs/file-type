use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_74: FileFormat = FileFormat {
    id: 74,
    source_type: SourceType::Pronom,
    name: "Wordperfect Secondary File",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
