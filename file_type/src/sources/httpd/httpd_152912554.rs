use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_152912554: FileType = FileType {
    file_format: &FileFormat {
        id: 152_912_554,
        source_type: SourceType::Httpd,
        name: "semf",
        extensions: &["semf"],
        media_types: &["application/vnd.semf"],
        signatures: &[],
        related_formats: &[],
    },
};
