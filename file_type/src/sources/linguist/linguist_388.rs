use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_388: FileType = FileType {
    file_format: &FileFormat {
        id: 388,
        source_type: SourceType::Linguist,
        name: "Vim Script",
        extensions: &["vba", "vim", "vimrc", "vmb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
