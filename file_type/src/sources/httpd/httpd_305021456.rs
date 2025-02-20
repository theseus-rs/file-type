use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_305021456: FileType = FileType {
    file_format: &FileFormat {
        id: 305_021_456,
        source_type: SourceType::Httpd,
        name: "novadigm edx",
        extensions: &["edx"],
        media_types: &["application/vnd.novadigm.edx"],
        signatures: &[],
        related_formats: &[],
    },
};
