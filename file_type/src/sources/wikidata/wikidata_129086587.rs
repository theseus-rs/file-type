use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129086587: FileType = FileType {
    file_format: &FileFormat {
        id: 129_086_587,
        source_type: SourceType::Wikidata,
        name: "Emacs Lisp file",
        extensions: &["el"],
        media_types: &["application/x-elisp", "text/x-elisp"],
        signatures: &[],
        related_formats: &[],
    },
};
