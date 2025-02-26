use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132634065: FileType = FileType {
    file_format: &FileFormat {
        id: 132_634_065,
        source_type: SourceType::Wikidata,
        name: "Crontab file format",
        extensions: &["tab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
