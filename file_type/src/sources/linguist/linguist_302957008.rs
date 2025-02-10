use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_302957008: FileType = FileType {
    file_format: &FileFormat {
        id: 302_957_008,
        source_type: SourceType::Linguist,
        name: "GN",
        extensions: &["gn", "gni"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
