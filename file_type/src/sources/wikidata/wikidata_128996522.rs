use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128996522: FileType = FileType {
    file_format: &FileFormat {
        id: 128_996_522,
        source_type: SourceType::Wikidata,
        name: "Easytrieve file format",
        extensions: &["ezt"],
        media_types: &["text/x-easytrieve"],
        signatures: &[],
        related_formats: &[],
    },
};
