use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4088672022: FileType = FileType {
    file_format: &FileFormat {
        id: 4_088_672_022,
        source_type: SourceType::Httpd,
        name: "cinderella",
        extensions: &["cdy"],
        media_types: &["application/vnd.cinderella"],
        signatures: &[],
        related_formats: &[],
    },
};
