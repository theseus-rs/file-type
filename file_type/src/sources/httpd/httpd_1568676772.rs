use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1568676772: FileType = FileType {
    file_format: &FileFormat {
        id: 1_568_676_772,
        source_type: SourceType::Httpd,
        name: "aiff",
        extensions: &["aif", "aiff", "aifc"],
        media_types: &["audio/x-aiff"],
        signatures: &[],
        related_formats: &[],
    },
};
