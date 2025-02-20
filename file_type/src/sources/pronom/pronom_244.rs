use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_244: FileType = FileType {
    file_format: &FileFormat {
        id: 244,
        source_type: SourceType::Pronom,
        name: "Inset Systems Bitmap",
        extensions: &["pix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
