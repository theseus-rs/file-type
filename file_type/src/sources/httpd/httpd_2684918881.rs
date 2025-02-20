use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2684918881: FileType = FileType {
    file_format: &FileFormat {
        id: 2_684_918_881,
        source_type: SourceType::Httpd,
        name: "collada xml",
        extensions: &["dae"],
        media_types: &["model/vnd.collada+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
