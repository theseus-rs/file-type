use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_348215381: FileType = FileType {
    file_format: &FileFormat {
        id: 348_215_381,
        source_type: SourceType::Httpd,
        name: "kde kchart",
        extensions: &["chrt"],
        media_types: &["application/vnd.kde.kchart"],
        signatures: &[],
        related_formats: &[],
    },
};
