use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_81525646: FileType = FileType {
    file_format: &FileFormat {
        id: 81_525_646,
        source_type: SourceType::Wikidata,
        name: "CorelDream 3D drawing",
        extensions: &["d3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
