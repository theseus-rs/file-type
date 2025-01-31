use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1369: FileFormat = FileFormat {
    id: 2_187,
    puid: "fmt/1369",
    name: "Error File",
    extensions: &["err"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
