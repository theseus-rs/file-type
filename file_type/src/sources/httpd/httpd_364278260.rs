use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_364278260: FileType = FileType {
    file_format: &FileFormat {
        id: 364_278_260,
        source_type: SourceType::Httpd,
        name: "xfig",
        extensions: &["fig"],
        media_types: &["application/x-xfig"],
        signatures: &[],
        related_formats: &[],
    },
};
