use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_320003511: FileType = FileType {
    file_format: &FileFormat {
        id: 320_003_511,
        source_type: SourceType::Httpd,
        name: "pn realaudio plugin",
        extensions: &["rmp"],
        media_types: &["audio/x-pn-realaudio-plugin"],
        signatures: &[],
        related_formats: &[],
    },
};
