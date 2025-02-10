use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_420: FileType = FileType {
    file_format: &FileFormat {
        id: 420,
        source_type: SourceType::Linguist,
        name: "wisp",
        extensions: &["wisp"],
        media_types: &["text/x-clojure"],
        signatures: &[],
        related_formats: &[],
    },
};
