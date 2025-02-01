use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1600: FileFormat = FileFormat {
    id: 2_427,
    puid: "fmt/1600",
    name: "ESRI ArcInfo DAT File (Internal)",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_421,
        relationship_type: RelationshipType::EquivalentTo,
    }],
};
