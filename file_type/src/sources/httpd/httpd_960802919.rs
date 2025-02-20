use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_960802919: FileType = FileType {
    file_format: &FileFormat {
        id: 960_802_919,
        source_type: SourceType::Httpd,
        name: "xara",
        extensions: &["xar"],
        media_types: &["application/vnd.xara"],
        signatures: &[],
        related_formats: &[],
    },
};
