use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1541299051: FileType = FileType {
    file_format: &FileFormat {
        id: 1_541_299_051,
        source_type: SourceType::Httpd,
        name: "oasis opendocument image template",
        extensions: &["oti"],
        media_types: &["application/vnd.oasis.opendocument.image-template"],
        signatures: &[],
        related_formats: &[],
    },
};
