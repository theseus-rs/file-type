use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_165: FileType = FileType {
    file_format: &FileFormat {
        id: 165,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &["vsd", "vst", "vss"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_229,
            },
        ],
    },
};
