use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_304: FileType = FileType {
    file_format: &FileFormat {
        id: 304,
        source_type: SourceType::Linguist,
        name: "Python traceback",
        extensions: &["pytb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
