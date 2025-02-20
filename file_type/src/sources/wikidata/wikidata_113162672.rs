use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113162672: FileType = FileType {
    file_format: &FileFormat {
        id: 113_162_672,
        source_type: SourceType::Wikidata,
        name: "MyContactManager File",
        extensions: &["mcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
