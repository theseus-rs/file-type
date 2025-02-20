use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_533608977: FileType = FileType {
    file_format: &FileFormat {
        id: 533_608_977,
        source_type: SourceType::Httpd,
        name: "patch ops error xml",
        extensions: &["xer"],
        media_types: &["application/patch-ops-error+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
