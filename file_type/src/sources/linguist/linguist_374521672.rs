use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_374521672: FileType = FileType {
    file_format: &FileFormat {
        id: 374_521_672,
        source_type: SourceType::Linguist,
        name: "WDL",
        extensions: &["wdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
