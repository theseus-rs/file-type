use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1398: FileFormat = FileFormat {
    id: 2_216,
    puid: "fmt/1398",
    name: "FARO WorkSpace File",
    extensions: &["fws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
