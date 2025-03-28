use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_475: FileType = FileType {
    file_format: &FileFormat {
        id: 475,
        source_type: SourceType::Pronom,
        name: "Dr Halo Bitmap",
        extensions: &["cut"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
