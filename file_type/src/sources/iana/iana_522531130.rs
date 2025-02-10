use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_522531130: FileType = FileType {
    file_format: &FileFormat {
        id: 522_531_130,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-package.relationships+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-package.relationships+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
