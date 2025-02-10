use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105852741: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_741,
        source_type: SourceType::Wikidata,
        name: "Snagit Windows Profile",
        extensions: &["snagprof"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
