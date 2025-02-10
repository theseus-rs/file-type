use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_812449347: FileType = FileType {
    file_format: &FileFormat {
        id: 812_449_347,
        source_type: SourceType::Httpd,
        name: "yamaha openscoreformat osfpvg xml",
        extensions: &["osfpvg"],
        media_types: &["application/vnd.yamaha.openscoreformat.osfpvg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
