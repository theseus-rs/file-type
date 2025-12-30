use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3920: FileType = FileType {
    file_format: &FileFormat {
        id: 3_920,
        source_type: SourceType::Pronom,
        name: "OpenDocument Spreadsheet",
        extensions: &["ods"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 778,
        }],
    },
};
