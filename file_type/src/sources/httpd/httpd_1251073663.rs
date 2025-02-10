use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1251073663: FileType = FileType {
    file_format: &FileFormat {
        id: 1_251_073_663,
        source_type: SourceType::Httpd,
        name: "nitf",
        extensions: &["ntf", "nitf"],
        media_types: &["application/vnd.nitf"],
        signatures: &[],
        related_formats: &[],
    },
};
