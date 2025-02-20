use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1389473401: FileType = FileType {
    file_format: &FileFormat {
        id: 1_389_473_401,
        source_type: SourceType::Httpd,
        name: "kde kpresenter",
        extensions: &["kpr", "kpt"],
        media_types: &["application/vnd.kde.kpresenter"],
        signatures: &[],
        related_formats: &[],
    },
};
