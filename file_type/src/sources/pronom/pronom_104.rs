use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_104: FileType = FileType {
    file_format: &FileFormat {
        id: 104,
        source_type: SourceType::Pronom,
        name: "AutoLISP File",
        extensions: &["lsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
