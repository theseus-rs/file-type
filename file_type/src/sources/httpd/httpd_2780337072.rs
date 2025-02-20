use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2780337072: FileType = FileType {
    file_format: &FileFormat {
        id: 2_780_337_072,
        source_type: SourceType::Httpd,
        name: "sfv",
        extensions: &["sfv"],
        media_types: &["text/x-sfv"],
        signatures: &[],
        related_formats: &[],
    },
};
