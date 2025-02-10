use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_325: FileType = FileType {
    file_format: &FileFormat {
        id: 325,
        source_type: SourceType::Linguist,
        name: "Rouge",
        extensions: &["rg"],
        media_types: &["text/x-clojure"],
        signatures: &[],
        related_formats: &[],
    },
};
