use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2057: FileType = FileType {
    file_format: &FileFormat {
        id: 2_057,
        source_type: SourceType::Pronom,
        name: "Band Interleaved By Pixel (BIP) Image Encoding",
        extensions: &["bip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
