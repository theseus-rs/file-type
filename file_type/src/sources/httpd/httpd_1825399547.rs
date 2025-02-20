use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1825399547: FileType = FileType {
    file_format: &FileFormat {
        id: 1_825_399_547,
        source_type: SourceType::Httpd,
        name: "mj2",
        extensions: &["mj2", "mjp2"],
        media_types: &["video/mj2"],
        signatures: &[],
        related_formats: &[],
    },
};
