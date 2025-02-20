use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_791486801: FileType = FileType {
    file_format: &FileFormat {
        id: 791_486_801,
        source_type: SourceType::Httpd,
        name: "exi",
        extensions: &["exi"],
        media_types: &["application/exi"],
        signatures: &[],
        related_formats: &[],
    },
};
