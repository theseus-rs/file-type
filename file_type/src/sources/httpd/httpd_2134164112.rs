use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2134164112: FileType = FileType {
    file_format: &FileFormat {
        id: 2_134_164_112,
        source_type: SourceType::Httpd,
        name: "ms application",
        extensions: &["application"],
        media_types: &["application/x-ms-application"],
        signatures: &[],
        related_formats: &[],
    },
};
