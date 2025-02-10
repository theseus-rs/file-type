use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2607: FileType = FileType {
    file_format: &FileFormat {
        id: 2_607,
        source_type: SourceType::Pronom,
        name: "Media Descriptor File",
        extensions: &["mdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
