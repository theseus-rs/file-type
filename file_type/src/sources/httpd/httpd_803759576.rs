use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_803759576: FileType = FileType {
    file_format: &FileFormat {
        id: 803_759_576,
        source_type: SourceType::Httpd,
        name: "mozilla xul xml",
        extensions: &["xul"],
        media_types: &["application/vnd.mozilla.xul+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
