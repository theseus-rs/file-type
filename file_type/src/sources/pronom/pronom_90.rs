use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_90: FileType = FileType {
    file_format: &FileFormat {
        id: 90,
        source_type: SourceType::Pronom,
        name: "Kodak FlashPix Image",
        extensions: &["fpx"],
        media_types: &["image/vnd.fpx"],
        signatures: &[],
        related_formats: &[],
    },
};
