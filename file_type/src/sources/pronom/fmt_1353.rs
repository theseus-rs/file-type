use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1353: FileFormat = FileFormat {
    id: 2_171,
    puid: "fmt/1353",
    name: "FamilyTree Maker Database",
    extensions: &["ftw", "fbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
