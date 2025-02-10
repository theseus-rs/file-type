use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_91493841: FileType = FileType {
    file_format: &FileFormat {
        id: 91_493_841,
        source_type: SourceType::Linguist,
        name: "Clarity",
        extensions: &["clar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
