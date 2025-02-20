use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2707319756: FileType = FileType {
    file_format: &FileFormat {
        id: 2_707_319_756,
        source_type: SourceType::Httpd,
        name: "wap wmlscriptc",
        extensions: &["wmlsc"],
        media_types: &["application/vnd.wap.wmlscriptc"],
        signatures: &[],
        related_formats: &[],
    },
};
