use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126326662: FileType = FileType {
    file_format: &FileFormat {
        id: 126_326_662,
        source_type: SourceType::Wikidata,
        name: "slrn configuration file",
        extensions: &["slrnrc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
