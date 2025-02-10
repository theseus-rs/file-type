use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3627480143: FileType = FileType {
    file_format: &FileFormat {
        id: 3_627_480_143,
        source_type: SourceType::Httpd,
        name: "webm",
        extensions: &["weba"],
        media_types: &["audio/webm"],
        signatures: &[],
        related_formats: &[],
    },
};
