use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1675515245: FileType = FileType {
    file_format: &FileFormat {
        id: 1_675_515_245,
        source_type: SourceType::Httpd,
        name: "blueice multipass",
        extensions: &["mpm"],
        media_types: &["application/vnd.blueice.multipass"],
        signatures: &[],
        related_formats: &[],
    },
};
