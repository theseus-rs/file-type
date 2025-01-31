use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_682: FileFormat = FileFormat {
    id: 1_481,
    puid: "fmt/682",
    name: "Thumbs DB file",
    extensions: &["db"],
    media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
    internal_signatures: &[],
    related_formats: &[],
};
