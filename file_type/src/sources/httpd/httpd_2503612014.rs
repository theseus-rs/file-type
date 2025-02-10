use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2503612014: FileType = FileType {
    file_format: &FileFormat {
        id: 2_503_612_014,
        source_type: SourceType::Httpd,
        name: "mobius msl",
        extensions: &["msl"],
        media_types: &["application/vnd.mobius.msl"],
        signatures: &[],
        related_formats: &[],
    },
};
