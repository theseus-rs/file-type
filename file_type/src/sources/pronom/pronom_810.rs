use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_810: FileFormat = FileFormat {
    id: 810,
    source_type: SourceType::Pronom,
    name: "JavaScript file",
    extensions: &["js"],
    media_types: &["application/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
