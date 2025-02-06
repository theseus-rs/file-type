use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1696: FileFormat = FileFormat {
    id: 1_696,
    source_type: SourceType::Pronom,
    name: "Compound WordPerfect for Windows Document",
    extensions: &["wpd", "doc", "wp6", "wp", "w60"],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[],
    related_formats: &[],
};
