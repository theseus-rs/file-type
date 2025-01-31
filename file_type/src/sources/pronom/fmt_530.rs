use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_530: FileFormat = FileFormat {
    id: 1_317,
    puid: "fmt/530",
    name: "eRuby HTML document",
    extensions: &["rhtml", "rhtm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
