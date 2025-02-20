use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2213303567: FileType = FileType {
    file_format: &FileFormat {
        id: 2_213_303_567,
        source_type: SourceType::Httpd,
        name: "openxmlformats officedocument spreadsheetml template",
        extensions: &["xltx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
        signatures: &[],
        related_formats: &[],
    },
};
