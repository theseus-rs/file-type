use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_353370408: FileType = FileType {
    file_format: &FileFormat {
        id: 353_370_408,
        source_type: SourceType::Iana,
        name: "vnd.nato.openxmlformats-package.iepd+zip",
        extensions: &[],
        media_types: &["application/vnd.nato.openxmlformats-package.iepd+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
