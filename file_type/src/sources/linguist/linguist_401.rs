use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_401: FileType = FileType {
    file_format: &FileFormat {
        id: 401,
        source_type: SourceType::Linguist,
        name: "XProc",
        extensions: &["xpl", "xproc"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
