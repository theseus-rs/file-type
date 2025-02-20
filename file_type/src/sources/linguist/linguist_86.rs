use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_86: FileType = FileType {
    file_format: &FileFormat {
        id: 86,
        source_type: SourceType::Linguist,
        name: "Darcs Patch",
        extensions: &["darcspatch", "dpatch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
