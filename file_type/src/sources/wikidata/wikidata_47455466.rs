use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47455466: FileType = FileType {
    file_format: &FileFormat {
        id: 47_455_466,
        source_type: SourceType::Wikidata,
        name: "Windows Portable Executable file format, 32-bit",
        extensions: &["dll", "exe", "sys"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
