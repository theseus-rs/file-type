use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_724: FileFormat = FileFormat {
    id: 1_523,
    puid: "fmt/724",
    name: "Keyhole Markup Language (Container)",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 975,
        relationship_type: RelationshipType::EquivalentTo,
    }],
};
