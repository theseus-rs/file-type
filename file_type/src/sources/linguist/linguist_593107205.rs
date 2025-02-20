use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_593107205: FileType = FileType {
    file_format: &FileFormat {
        id: 593_107_205,
        source_type: SourceType::Linguist,
        name: "QuickBASIC",
        extensions: &["bas"],
        media_types: &["text/x-vb"],
        signatures: &[],
        related_formats: &[],
    },
};
