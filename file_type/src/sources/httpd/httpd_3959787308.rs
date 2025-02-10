use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3959787308: FileType = FileType {
    file_format: &FileFormat {
        id: 3_959_787_308,
        source_type: SourceType::Httpd,
        name: "dece unspecified",
        extensions: &["uvx", "uvvx"],
        media_types: &["application/vnd.dece.unspecified"],
        signatures: &[],
        related_formats: &[],
    },
};
