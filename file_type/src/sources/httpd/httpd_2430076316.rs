use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2430076316: FileType = FileType {
    file_format: &FileFormat {
        id: 2_430_076_316,
        source_type: SourceType::Httpd,
        name: "fujitsu oasys",
        extensions: &["oas"],
        media_types: &["application/vnd.fujitsu.oasys"],
        signatures: &[],
        related_formats: &[],
    },
};
