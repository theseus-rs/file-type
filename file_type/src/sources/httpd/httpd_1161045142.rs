use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1161045142: FileType = FileType {
    file_format: &FileFormat {
        id: 1_161_045_142,
        source_type: SourceType::Httpd,
        name: "yang",
        extensions: &["yang"],
        media_types: &["application/yang"],
        signatures: &[],
        related_formats: &[],
    },
};
