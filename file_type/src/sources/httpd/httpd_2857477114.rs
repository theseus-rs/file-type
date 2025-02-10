use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2857477114: FileType = FileType {
    file_format: &FileFormat {
        id: 2_857_477_114,
        source_type: SourceType::Httpd,
        name: "kahootz",
        extensions: &["ktz", "ktr"],
        media_types: &["application/vnd.kahootz"],
        signatures: &[],
        related_formats: &[],
    },
};
