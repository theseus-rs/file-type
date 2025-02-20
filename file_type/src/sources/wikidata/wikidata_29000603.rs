use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000603: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_603,
        source_type: SourceType::Wikidata,
        name: "Windows Registry policy file",
        extensions: &["pol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
