use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1352: FileFormat = FileFormat {
    id: 2_170,
    puid: "fmt/1352",
    name: "FamilyTree Maker Database",
    extensions: &["ftw", "fbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
