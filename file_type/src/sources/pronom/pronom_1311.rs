use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1311: FileFormat = FileFormat {
    id: 1_311,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Theme",
    extensions: &["thmx"],
    media_types: &["application/vnd.ms-officetheme"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 941,
    }],
};
