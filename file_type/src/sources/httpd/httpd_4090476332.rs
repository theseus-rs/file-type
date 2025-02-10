use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4090476332: FileType = FileType {
    file_format: &FileFormat {
        id: 4_090_476_332,
        source_type: SourceType::Httpd,
        name: "oasis opendocument database",
        extensions: &["odb"],
        media_types: &["application/vnd.oasis.opendocument.database"],
        signatures: &[],
        related_formats: &[],
    },
};
