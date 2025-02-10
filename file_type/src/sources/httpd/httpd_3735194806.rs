use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3735194806: FileType = FileType {
    file_format: &FileFormat {
        id: 3_735_194_806,
        source_type: SourceType::Httpd,
        name: "sema",
        extensions: &["sema"],
        media_types: &["application/vnd.sema"],
        signatures: &[],
        related_formats: &[],
    },
};
