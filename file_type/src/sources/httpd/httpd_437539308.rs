use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_437539308: FileType = FileType {
    file_format: &FileFormat {
        id: 437_539_308,
        source_type: SourceType::Httpd,
        name: "cfs compressed",
        extensions: &["cfs"],
        media_types: &["application/x-cfs-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
