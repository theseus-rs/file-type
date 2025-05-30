use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_482286266: FileType = FileType {
    file_format: &FileFormat {
        id: 482_286_266,
        source_type: SourceType::Httpd,
        name: "marcxml xml",
        extensions: &["mrcx"],
        media_types: &["application/marcxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
