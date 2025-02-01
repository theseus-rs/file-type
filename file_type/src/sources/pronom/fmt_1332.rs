use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1332: FileFormat = FileFormat {
    id: 2_150,
    puid: "fmt/1332",
    name: "HP Photo Album",
    extensions: &["albm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
