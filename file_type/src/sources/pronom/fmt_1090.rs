use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1090: FileFormat = FileFormat {
    id: 1_898,
    puid: "fmt/1090",
    name: "Exclude File",
    extensions: &["exclude"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
