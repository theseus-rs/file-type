use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3207609300: FileType = FileType {
    file_format: &FileFormat {
        id: 3_207_609_300,
        source_type: SourceType::Httpd,
        name: "wt stf",
        extensions: &["stf"],
        media_types: &["application/vnd.wt.stf"],
        signatures: &[],
        related_formats: &[],
    },
};
