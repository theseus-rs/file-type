use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3225988671: FileType = FileType {
    file_format: &FileFormat {
        id: 3_225_988_671,
        source_type: SourceType::Httpd,
        name: "3gpp2",
        extensions: &["3g2"],
        media_types: &["video/3gpp2"],
        signatures: &[],
        related_formats: &[],
    },
};
