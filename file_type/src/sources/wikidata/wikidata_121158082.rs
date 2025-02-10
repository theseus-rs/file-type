use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121158082: FileType = FileType {
    file_format: &FileFormat {
        id: 121_158_082,
        source_type: SourceType::Wikidata,
        name: "ResumeMaker file",
        extensions: &["rmr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
