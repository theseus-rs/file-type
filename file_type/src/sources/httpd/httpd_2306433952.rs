use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2306433952: FileType = FileType {
    file_format: &FileFormat {
        id: 2_306_433_952,
        source_type: SourceType::Httpd,
        name: "mobius dis",
        extensions: &["dis"],
        media_types: &["application/vnd.mobius.dis"],
        signatures: &[],
        related_formats: &[],
    },
};
