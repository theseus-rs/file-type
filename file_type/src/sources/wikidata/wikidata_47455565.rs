use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47455565: FileType = FileType {
    file_format: &FileFormat {
        id: 47_455_565,
        source_type: SourceType::Wikidata,
        name: "Windows Portable Executable file format, 64-bit",
        extensions: &["coff", "dll", "exe", "sys"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
