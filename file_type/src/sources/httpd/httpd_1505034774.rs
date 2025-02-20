use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1505034774: FileType = FileType {
    file_format: &FileFormat {
        id: 1_505_034_774,
        source_type: SourceType::Httpd,
        name: "micrografx flo",
        extensions: &["flo"],
        media_types: &["application/vnd.micrografx.flo"],
        signatures: &[],
        related_formats: &[],
    },
};
