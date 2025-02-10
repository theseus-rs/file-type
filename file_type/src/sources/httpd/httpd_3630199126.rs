use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3630199126: FileType = FileType {
    file_format: &FileFormat {
        id: 3_630_199_126,
        source_type: SourceType::Httpd,
        name: "vcx",
        extensions: &["vcx"],
        media_types: &["application/vnd.vcx"],
        signatures: &[],
        related_formats: &[],
    },
};
