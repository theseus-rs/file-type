use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_208: FileType = FileType {
    file_format: &FileFormat {
        id: 208,
        source_type: SourceType::Linguist,
        name: "LiveScript",
        extensions: &["_ls", "ls"],
        media_types: &["text/x-livescript"],
        signatures: &[],
        related_formats: &[],
    },
};
