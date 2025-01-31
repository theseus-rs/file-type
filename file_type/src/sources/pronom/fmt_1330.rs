use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1330: FileFormat = FileFormat {
    id: 2_148,
    puid: "fmt/1330",
    name: "Avery DesignPro Document",
    extensions: &["zdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
