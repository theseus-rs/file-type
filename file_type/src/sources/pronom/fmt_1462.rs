use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1462: FileFormat = FileFormat {
    id: 2_285,
    puid: "fmt/1462",
    name: "Comic Book Archive",
    extensions: &["cb7", "cba", "cbr", "cbt", "cbz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
