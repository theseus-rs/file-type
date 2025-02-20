use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3742064704: FileType = FileType {
    file_format: &FileFormat {
        id: 3_742_064_704,
        source_type: SourceType::Httpd,
        name: "osgi subsystem",
        extensions: &["esa"],
        media_types: &["application/vnd.osgi.subsystem"],
        signatures: &[],
        related_formats: &[],
    },
};
