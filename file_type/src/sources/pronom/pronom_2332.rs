use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2332: FileType = FileType {
    file_format: &FileFormat {
        id: 2_332,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &["vsd", "vst", "vss"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_229,
        }],
    },
};
