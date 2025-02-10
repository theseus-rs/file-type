use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2662883759: FileType = FileType {
    file_format: &FileFormat {
        id: 2_662_883_759,
        source_type: SourceType::Httpd,
        name: "adobe photoshop",
        extensions: &["psd"],
        media_types: &["image/vnd.adobe.photoshop"],
        signatures: &[],
        related_formats: &[],
    },
};
