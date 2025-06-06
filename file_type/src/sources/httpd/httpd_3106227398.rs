use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3106227398: FileType = FileType {
    file_format: &FileFormat {
        id: 3_106_227_398,
        source_type: SourceType::Httpd,
        name: "stardivision impress",
        extensions: &["sdd"],
        media_types: &["application/vnd.stardivision.impress"],
        signatures: &[],
        related_formats: &[],
    },
};
