use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_676218789: FileType = FileType {
    file_format: &FileFormat {
        id: 676_218_789,
        source_type: SourceType::Httpd,
        name: "dgc compressed",
        extensions: &["dgc"],
        media_types: &["application/x-dgc-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
