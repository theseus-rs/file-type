use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_970323759: FileType = FileType {
    file_format: &FileFormat {
        id: 970_323_759,
        source_type: SourceType::Httpd,
        name: "debian package",
        extensions: &["deb", "udeb"],
        media_types: &["application/x-debian-package"],
        signatures: &[],
        related_formats: &[],
    },
};
