use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3593012719: FileType = FileType {
    file_format: &FileFormat {
        id: 3_593_012_719,
        source_type: SourceType::Httpd,
        name: "ms vob",
        extensions: &["vob"],
        media_types: &["video/x-ms-vob"],
        signatures: &[],
        related_formats: &[],
    },
};
