use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2723193037: FileType = FileType {
    file_format: &FileFormat {
        id: 2_723_193_037,
        source_type: SourceType::Httpd,
        name: "kde kformula",
        extensions: &["kfo"],
        media_types: &["application/vnd.kde.kformula"],
        signatures: &[],
        related_formats: &[],
    },
};
