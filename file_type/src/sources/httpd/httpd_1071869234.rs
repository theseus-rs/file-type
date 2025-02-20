use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1071869234: FileType = FileType {
    file_format: &FileFormat {
        id: 1_071_869_234,
        source_type: SourceType::Httpd,
        name: "relax ng compact syntax",
        extensions: &["rnc"],
        media_types: &["application/relax-ng-compact-syntax"],
        signatures: &[],
        related_formats: &[],
    },
};
