use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2198329675: FileType = FileType {
    file_format: &FileFormat {
        id: 2_198_329_675,
        source_type: SourceType::Httpd,
        name: "caf",
        extensions: &["caf"],
        media_types: &["audio/x-caf"],
        signatures: &[],
        related_formats: &[],
    },
};
