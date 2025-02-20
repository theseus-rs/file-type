use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_523: FileType = FileType {
    file_format: &FileFormat {
        id: 523,
        source_type: SourceType::Pronom,
        name: "Scanstudio 16-Colour Bitmap",
        extensions: &["adc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
