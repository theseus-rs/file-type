use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1846: FileFormat = FileFormat {
    id: 2_698,
    puid: "fmt/1846",
    name: "Fountain Markup Language File",
    extensions: &["spmd", "fountain"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
