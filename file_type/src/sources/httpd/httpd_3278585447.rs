use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3278585447: FileType = FileType {
    file_format: &FileFormat {
        id: 3_278_585_447,
        source_type: SourceType::Httpd,
        name: "groove vcard",
        extensions: &["vcg"],
        media_types: &["application/vnd.groove-vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
