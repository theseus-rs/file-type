use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2551238913: FileType = FileType {
    file_format: &FileFormat {
        id: 2_551_238_913,
        source_type: SourceType::Httpd,
        name: "prs btif",
        extensions: &["btif"],
        media_types: &["image/prs.btif"],
        signatures: &[],
        related_formats: &[],
    },
};
