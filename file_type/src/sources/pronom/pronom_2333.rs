use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2333: FileType = FileType {
    file_format: &FileFormat {
        id: 2_333,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &["vsd", "vst", "vss", "vsw"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_229,
        }],
    },
};
