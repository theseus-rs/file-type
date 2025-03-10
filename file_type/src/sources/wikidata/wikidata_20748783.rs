use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_20748783: FileType = FileType {
    file_format: &FileFormat {
        id: 20_748_783,
        source_type: SourceType::Wikidata,
        name: "Makefile",
        extensions: &[],
        media_types: &["text/x-makefile"],
        signatures: &[],
        related_formats: &[],
    },
};
