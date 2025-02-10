use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3288976212: FileType = FileType {
    file_format: &FileFormat {
        id: 3_288_976_212,
        source_type: SourceType::Httpd,
        name: "pmi widget",
        extensions: &["wg"],
        media_types: &["application/vnd.pmi.widget"],
        signatures: &[],
        related_formats: &[],
    },
};
