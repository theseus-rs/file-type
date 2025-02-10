use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3460806808: FileType = FileType {
    file_format: &FileFormat {
        id: 3_460_806_808,
        source_type: SourceType::Httpd,
        name: "xpixmap",
        extensions: &["xpm"],
        media_types: &["image/x-xpixmap"],
        signatures: &[],
        related_formats: &[],
    },
};
