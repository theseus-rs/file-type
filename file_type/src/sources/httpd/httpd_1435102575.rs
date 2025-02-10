use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1435102575: FileType = FileType {
    file_format: &FileFormat {
        id: 1_435_102_575,
        source_type: SourceType::Httpd,
        name: "yamaha hv dic",
        extensions: &["hvd"],
        media_types: &["application/vnd.yamaha.hv-dic"],
        signatures: &[],
        related_formats: &[],
    },
};
