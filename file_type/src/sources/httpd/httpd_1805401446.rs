use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1805401446: FileType = FileType {
    file_format: &FileFormat {
        id: 1_805_401_446,
        source_type: SourceType::Httpd,
        name: "authorware map",
        extensions: &["aam"],
        media_types: &["application/x-authorware-map"],
        signatures: &[],
        related_formats: &[],
    },
};
