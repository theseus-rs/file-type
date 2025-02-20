use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
