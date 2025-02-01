use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1755: FileFormat = FileFormat {
    id: 2_603,
    puid: "fmt/1755",
    name: "OpenDocument Spreadsheet",
    extensions: &["ods"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_037,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
