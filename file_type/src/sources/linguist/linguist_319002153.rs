use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_319002153: FileType = FileType {
    file_format: &FileFormat {
        id: 319_002_153,
        source_type: SourceType::Linguist,
        name: "ReasonLIGO",
        extensions: &["religo"],
        media_types: &["text/x-rustsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
