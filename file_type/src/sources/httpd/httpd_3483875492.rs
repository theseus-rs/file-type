use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3483875492: FileType = FileType {
    file_format: &FileFormat {
        id: 3_483_875_492,
        source_type: SourceType::Httpd,
        name: "fujitsu oasys2",
        extensions: &["oa2"],
        media_types: &["application/vnd.fujitsu.oasys2"],
        signatures: &[],
        related_formats: &[],
    },
};
