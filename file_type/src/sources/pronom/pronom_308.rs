use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_308: FileFormat = FileFormat {
    id: 308,
    source_type: SourceType::Pronom,
    name: "Adobe ACD",
    extensions: &["acd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
