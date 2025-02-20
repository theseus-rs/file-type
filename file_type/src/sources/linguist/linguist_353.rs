use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_353: FileType = FileType {
    file_format: &FileFormat {
        id: 353,
        source_type: SourceType::Linguist,
        name: "Smarty",
        extensions: &["tpl"],
        media_types: &["text/x-smarty"],
        signatures: &[],
        related_formats: &[],
    },
};
