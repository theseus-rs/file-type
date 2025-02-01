use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1331: FileFormat = FileFormat {
    id: 2_149,
    puid: "fmt/1331",
    name: "Avery DesignPro Document",
    extensions: &["zdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
