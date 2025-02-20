use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4293215786: FileType = FileType {
    file_format: &FileFormat {
        id: 4_293_215_786,
        source_type: SourceType::Httpd,
        name: "acucobol",
        extensions: &["acu"],
        media_types: &["application/vnd.acucobol"],
        signatures: &[],
        related_formats: &[],
    },
};
