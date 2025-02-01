use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1190: FileFormat = FileFormat {
    id: 2_000,
    puid: "fmt/1190",
    name: "Adobe SWC Package",
    extensions: &["swc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
