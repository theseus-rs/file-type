use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2290373666: FileType = FileType {
    file_format: &FileFormat {
        id: 2_290_373_666,
        source_type: SourceType::Httpd,
        name: "dtbncx xml",
        extensions: &["ncx"],
        media_types: &["application/x-dtbncx+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
