use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2758758858: FileType = FileType {
    file_format: &FileFormat {
        id: 2_758_758_858,
        source_type: SourceType::Httpd,
        name: "cmx",
        extensions: &["cmx"],
        media_types: &["image/x-cmx"],
        signatures: &[],
        related_formats: &[],
    },
};
