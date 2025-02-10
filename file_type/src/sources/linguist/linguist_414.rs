use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_414: FileType = FileType {
    file_format: &FileFormat {
        id: 414,
        source_type: SourceType::Linguist,
        name: "edn",
        extensions: &["edn"],
        media_types: &["text/x-clojure"],
        signatures: &[],
        related_formats: &[],
    },
};
