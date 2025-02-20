use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2780337060: FileType = FileType {
    file_format: &FileFormat {
        id: 2_780_337_060,
        source_type: SourceType::Httpd,
        name: "mfer",
        extensions: &["mwf"],
        media_types: &["application/vnd.mfer"],
        signatures: &[],
        related_formats: &[],
    },
};
