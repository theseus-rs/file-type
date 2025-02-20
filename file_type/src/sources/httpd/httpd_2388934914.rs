use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2388934914: FileType = FileType {
    file_format: &FileFormat {
        id: 2_388_934_914,
        source_type: SourceType::Httpd,
        name: "cbr",
        extensions: &["cbr", "cba", "cbt", "cbz", "cb7"],
        media_types: &["application/x-cbr"],
        signatures: &[],
        related_formats: &[],
    },
};
