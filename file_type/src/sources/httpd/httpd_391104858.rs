use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_391104858: FileType = FileType {
    file_format: &FileFormat {
        id: 391_104_858,
        source_type: SourceType::Httpd,
        name: "reginfo xml",
        extensions: &["rif"],
        media_types: &["application/reginfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
