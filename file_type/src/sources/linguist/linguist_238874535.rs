use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_238874535: FileType = FileType {
    file_format: &FileFormat {
        id: 238_874_535,
        source_type: SourceType::Linguist,
        name: "MiniZinc",
        extensions: &["mzn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
