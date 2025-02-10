use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2325964356: FileType = FileType {
    file_format: &FileFormat {
        id: 2_325_964_356,
        source_type: SourceType::Httpd,
        name: "c",
        extensions: &["c", "cc", "cxx", "cpp", "h", "hh", "dic"],
        media_types: &["text/x-c"],
        signatures: &[],
        related_formats: &[],
    },
};
