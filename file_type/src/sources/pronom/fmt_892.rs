use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_892: FileFormat = FileFormat {
    id: 1_696,
    puid: "fmt/892",
    name: "Compound WordPerfect for Windows Document",
    extensions: &["wpd", "doc", "wp6", "wp", "w60"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[],
};
