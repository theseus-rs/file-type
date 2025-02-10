use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1049248689: FileType = FileType {
    file_format: &FileFormat {
        id: 1_049_248_689,
        source_type: SourceType::Httpd,
        name: "dece hd",
        extensions: &["uvh", "uvvh"],
        media_types: &["video/vnd.dece.hd"],
        signatures: &[],
        related_formats: &[],
    },
};
