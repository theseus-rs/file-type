use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2764967758: FileType = FileType {
    file_format: &FileFormat {
        id: 2_764_967_758,
        source_type: SourceType::Httpd,
        name: "scvp cv request",
        extensions: &["scq"],
        media_types: &["application/scvp-cv-request"],
        signatures: &[],
        related_formats: &[],
    },
};
