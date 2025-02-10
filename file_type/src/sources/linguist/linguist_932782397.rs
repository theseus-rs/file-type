use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_932782397: FileType = FileType {
    file_format: &FileFormat {
        id: 932_782_397,
        source_type: SourceType::Linguist,
        name: "Marko",
        extensions: &["marko"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
