use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1218352227: FileType = FileType {
    file_format: &FileFormat {
        id: 1_218_352_227,
        source_type: SourceType::Httpd,
        name: "realvnc bed",
        extensions: &["bed"],
        media_types: &["application/vnd.realvnc.bed"],
        signatures: &[],
        related_formats: &[],
    },
};
