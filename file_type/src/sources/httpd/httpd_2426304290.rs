use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2426304290: FileType = FileType {
    file_format: &FileFormat {
        id: 2_426_304_290,
        source_type: SourceType::Httpd,
        name: "nokia n gage data",
        extensions: &["ngdat"],
        media_types: &["application/vnd.nokia.n-gage.data"],
        signatures: &[],
        related_formats: &[],
    },
};
