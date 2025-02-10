use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105861238: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_238,
        source_type: SourceType::Wikidata,
        name: "Camtasia Zipped Library",
        extensions: &["libzip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
