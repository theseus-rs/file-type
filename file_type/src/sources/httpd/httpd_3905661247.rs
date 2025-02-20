use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3905661247: FileType = FileType {
    file_format: &FileFormat {
        id: 3_905_661_247,
        source_type: SourceType::Httpd,
        name: "oasis opendocument graphics",
        extensions: &["odg"],
        media_types: &["application/vnd.oasis.opendocument.graphics"],
        signatures: &[],
        related_formats: &[],
    },
};
