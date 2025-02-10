use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4019441519: FileType = FileType {
    file_format: &FileFormat {
        id: 4_019_441_519,
        source_type: SourceType::Httpd,
        name: "airzip filesecure azs",
        extensions: &["azs"],
        media_types: &["application/vnd.airzip.filesecure.azs"],
        signatures: &[],
        related_formats: &[],
    },
};
