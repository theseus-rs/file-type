use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1570593: FileType = FileType {
    file_format: &FileFormat {
        id: 1_570_593,
        source_type: SourceType::Wikidata,
        name: "Ogg Media",
        extensions: &["ogm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
