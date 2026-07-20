use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_492781155: FileType = FileType {
    file_format: &FileFormat {
        id: 492_781_155,
        source_type: SourceType::Linguist,
        name: "OverPy",
        extensions: &["opy"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
