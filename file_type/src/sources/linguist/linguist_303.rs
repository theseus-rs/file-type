use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_303: FileType = FileType {
    file_format: &FileFormat {
        id: 303,
        source_type: SourceType::Linguist,
        name: "Python",
        extensions: &[
            "cgi", "fcgi", "gyp", "gypi", "lmi", "py", "py3", "pyde", "pyi", "pyp", "pyt", "pyw",
            "rpy", "spec", "tac", "wsgi", "xpy",
        ],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
