use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2208184241: FileType = FileType {
    file_format: &FileFormat {
        id: 2_208_184_241,
        source_type: SourceType::Httpd,
        name: "rsd xml",
        extensions: &["rsd"],
        media_types: &["application/rsd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
