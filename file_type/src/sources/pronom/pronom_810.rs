use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_810: FileFormat = FileFormat {
    id: 810,
    source_type: SourceType::Pronom,
    name: "JavaScript file",
    extensions: &["js"],
    media_types: &["application/javascript"],
    signatures: &[],
    related_formats: &[],
};
