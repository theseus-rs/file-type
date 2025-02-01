use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_886: FileFormat = FileFormat {
    id: 1_690,
    puid: "fmt/886",
    name: "HTML Components",
    extensions: &["htc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
