use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1122188876: FileType = FileType {
    file_format: &FileFormat {
        id: 1_122_188_876,
        source_type: SourceType::Httpd,
        name: "dra",
        extensions: &["dra"],
        media_types: &["audio/vnd.dra"],
        signatures: &[],
        related_formats: &[],
    },
};
