use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_571581390: FileType = FileType {
    file_format: &FileFormat {
        id: 571_581_390,
        source_type: SourceType::Httpd,
        name: "accpac simply imp",
        extensions: &["imp"],
        media_types: &["application/vnd.accpac.simply.imp"],
        signatures: &[],
        related_formats: &[],
    },
};
