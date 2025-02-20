use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_43235474: FileType = FileType {
    file_format: &FileFormat {
        id: 43_235_474,
        source_type: SourceType::Httpd,
        name: "oasis opendocument graphics template",
        extensions: &["otg"],
        media_types: &["application/vnd.oasis.opendocument.graphics-template"],
        signatures: &[],
        related_formats: &[],
    },
};
