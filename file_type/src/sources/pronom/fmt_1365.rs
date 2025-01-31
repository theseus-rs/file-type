use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1365: FileFormat = FileFormat {
    id: 2_183,
    puid: "fmt/1365",
    name: "Debug File",
    extensions: &["dbg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
