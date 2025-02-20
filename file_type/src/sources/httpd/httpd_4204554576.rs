use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4204554576: FileType = FileType {
    file_format: &FileFormat {
        id: 4_204_554_576,
        source_type: SourceType::Httpd,
        name: "ms project",
        extensions: &["mpp", "mpt"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
