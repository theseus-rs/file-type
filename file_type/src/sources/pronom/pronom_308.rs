use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_308: FileFormat = FileFormat {
    id: 308,
    source_type: SourceType::Pronom,
    name: "Adobe ACD",
    extensions: &["acd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
