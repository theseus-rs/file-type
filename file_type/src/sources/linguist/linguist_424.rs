use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_424: FileType = FileType {
    file_format: &FileFormat {
        id: 424,
        source_type: SourceType::Linguist,
        name: "CSON",
        extensions: &["cson"],
        media_types: &["text/x-coffeescript"],
        signatures: &[],
        related_formats: &[],
    },
};
