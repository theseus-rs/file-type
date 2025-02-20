use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3433652685: FileType = FileType {
    file_format: &FileFormat {
        id: 3_433_652_685,
        source_type: SourceType::Httpd,
        name: "openxmlformats officedocument presentationml slideshow",
        extensions: &["ppsx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
        signatures: &[],
        related_formats: &[],
    },
};
