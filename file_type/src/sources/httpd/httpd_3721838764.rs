use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3721838764: FileType = FileType {
    file_format: &FileFormat {
        id: 3_721_838_764,
        source_type: SourceType::Httpd,
        name: "shana informed package",
        extensions: &["ipk"],
        media_types: &["application/vnd.shana.informed.package"],
        signatures: &[],
        related_formats: &[],
    },
};
