use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3039970627: FileType = FileType {
    file_format: &FileFormat {
        id: 3_039_970_627,
        source_type: SourceType::Httpd,
        name: "net fpx",
        extensions: &["npx"],
        media_types: &["image/vnd.net-fpx"],
        signatures: &[],
        related_formats: &[],
    },
};
