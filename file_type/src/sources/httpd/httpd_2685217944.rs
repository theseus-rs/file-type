use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2685217944: FileType = FileType {
    file_format: &FileFormat {
        id: 2_685_217_944,
        source_type: SourceType::Httpd,
        name: "palm",
        extensions: &["pdb", "pqa", "oprc"],
        media_types: &["application/vnd.palm"],
        signatures: &[],
        related_formats: &[],
    },
};
