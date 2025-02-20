use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1685005414: FileType = FileType {
    file_format: &FileFormat {
        id: 1_685_005_414,
        source_type: SourceType::Httpd,
        name: "tab separated values",
        extensions: &["tsv"],
        media_types: &["text/tab-separated-values"],
        signatures: &[],
        related_formats: &[],
    },
};
