use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1428: FileType = FileType {
    file_format: &FileFormat {
        id: 1_428,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Show",
        extensions: &["ppsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 941,
        }],
    },
};
