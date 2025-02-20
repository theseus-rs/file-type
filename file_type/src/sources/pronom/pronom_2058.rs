use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2058: FileType = FileType {
    file_format: &FileFormat {
        id: 2_058,
        source_type: SourceType::Pronom,
        name: "Band Sequential (BSQ) Image Encoding",
        extensions: &["bsq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
