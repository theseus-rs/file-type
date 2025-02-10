use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_575143428: FileType = FileType {
    file_format: &FileFormat {
        id: 575_143_428,
        source_type: SourceType::Linguist,
        name: "ImageJ Macro",
        extensions: &["ijm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
