use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_8229684: FileType = FileType {
    file_format: &FileFormat {
        id: 8_229_684,
        source_type: SourceType::Wikidata,
        name: "Elf",
        extensions: &["elf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
