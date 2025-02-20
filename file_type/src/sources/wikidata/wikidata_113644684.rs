use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113644684: FileType = FileType {
    file_format: &FileFormat {
        id: 113_644_684,
        source_type: SourceType::Wikidata,
        name: "Ulead File For Photo Projects",
        extensions: &["ufp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
