use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_46342762: FileType = FileType {
    file_format: &FileFormat {
        id: 46_342_762,
        source_type: SourceType::Httpd,
        name: "mp2t",
        extensions: &["ts", "m2t", "m2ts", "mts"],
        media_types: &["video/mp2t"],
        signatures: &[],
        related_formats: &[],
    },
};
