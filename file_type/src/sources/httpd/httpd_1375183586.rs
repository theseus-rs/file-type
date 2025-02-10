use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1375183586: FileType = FileType {
    file_format: &FileFormat {
        id: 1_375_183_586,
        source_type: SourceType::Httpd,
        name: "rim cod",
        extensions: &["cod"],
        media_types: &["application/vnd.rim.cod"],
        signatures: &[],
        related_formats: &[],
    },
};
