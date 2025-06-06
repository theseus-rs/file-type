use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_102: FileType = FileType {
    file_format: &FileFormat {
        id: 102,
        source_type: SourceType::Linguist,
        name: "Emacs Lisp",
        extensions: &["el", "emacs", "emacs.desktop"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
