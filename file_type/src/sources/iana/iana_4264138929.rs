use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4264138929: FileType = FileType {
    file_format: &FileFormat {
        id: 4_264_138_929,
        source_type: SourceType::Iana,
        name: "pwg-raster",
        extensions: &[],
        media_types: &["image/pwg-raster"],
        signatures: &[],
        related_formats: &[],
    },
};
