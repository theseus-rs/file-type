use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111341451: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_451,
        source_type: SourceType::Wikidata,
        name: "ScreamTracker v3 sample",
        extensions: &["s3i"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
