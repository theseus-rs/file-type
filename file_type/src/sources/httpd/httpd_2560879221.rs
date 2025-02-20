use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2560879221: FileType = FileType {
    file_format: &FileFormat {
        id: 2_560_879_221,
        source_type: SourceType::Httpd,
        name: "kde kspread",
        extensions: &["ksp"],
        media_types: &["application/vnd.kde.kspread"],
        signatures: &[],
        related_formats: &[],
    },
};
