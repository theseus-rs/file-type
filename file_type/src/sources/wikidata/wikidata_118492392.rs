use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118492392: FileType = FileType {
    file_format: &FileFormat {
        id: 118_492_392,
        source_type: SourceType::Wikidata,
        name: "Quicken 3 Database File",
        extensions: &["qst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
