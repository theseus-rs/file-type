use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_524: FileFormat = FileFormat {
    id: 1_311,
    puid: "fmt/524",
    name: "Microsoft Office Theme",
    extensions: &["thmx"],
    media_types: &["application/vnd.ms-officetheme"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
