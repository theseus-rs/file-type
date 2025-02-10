use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3433652685: FileType = FileType {
    file_format: &FileFormat {
        id: 3_433_652_685,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.slideshow",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
        signatures: &[],
        related_formats: &[],
    },
};
