use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1314711206: FileType = FileType {
    file_format: &FileFormat {
        id: 1_314_711_206,
        source_type: SourceType::Httpd,
        name: "pvi ptid1",
        extensions: &["ptid"],
        media_types: &["application/vnd.pvi.ptid1"],
        signatures: &[],
        related_formats: &[],
    },
};
