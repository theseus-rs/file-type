use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2511595652: FileType = FileType {
    file_format: &FileFormat {
        id: 2_511_595_652,
        source_type: SourceType::Httpd,
        name: "mie",
        extensions: &["mie"],
        media_types: &["application/x-mie"],
        signatures: &[],
        related_formats: &[],
    },
};
