use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2125042282: FileType = FileType {
    file_format: &FileFormat {
        id: 2_125_042_282,
        source_type: SourceType::Httpd,
        name: "ibm rights management",
        extensions: &["irm"],
        media_types: &["application/vnd.ibm.rights-management"],
        signatures: &[],
        related_formats: &[],
    },
};
