use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_213517500: FileType = FileType {
    file_format: &FileFormat {
        id: 213_517_500,
        source_type: SourceType::Httpd,
        name: "mophun certificate",
        extensions: &["mpc"],
        media_types: &["application/vnd.mophun.certificate"],
        signatures: &[],
        related_formats: &[],
    },
};
