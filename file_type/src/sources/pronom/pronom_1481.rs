use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1481: FileFormat = FileFormat {
    id: 1_481,
    source_type: SourceType::Pronom,
    name: "Thumbs DB file",
    extensions: &["db"],
    media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
    signatures: &[],
    related_formats: &[],
};
