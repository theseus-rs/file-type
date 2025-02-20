use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113354835: FileType = FileType {
    file_format: &FileFormat {
        id: 113_354_835,
        source_type: SourceType::Wikidata,
        name: "Snagit Preset file",
        extensions: &["snagpresets"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
