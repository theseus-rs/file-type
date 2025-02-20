use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3729972083: FileType = FileType {
    file_format: &FileFormat {
        id: 3_729_972_083,
        source_type: SourceType::Httpd,
        name: "mrsid image",
        extensions: &["sid"],
        media_types: &["image/x-mrsid-image"],
        signatures: &[],
        related_formats: &[],
    },
};
