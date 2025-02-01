use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_941: FileFormat = FileFormat {
    id: 1_746,
    puid: "fmt/941",
    name: "Back Up File",
    extensions: &["bak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
