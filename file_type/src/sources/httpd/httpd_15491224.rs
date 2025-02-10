use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_15491224: FileType = FileType {
    file_format: &FileFormat {
        id: 15_491_224,
        source_type: SourceType::Httpd,
        name: "gmx",
        extensions: &["gmx"],
        media_types: &["application/vnd.gmx"],
        signatures: &[],
        related_formats: &[],
    },
};
