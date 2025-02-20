use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2620857507: FileType = FileType {
    file_format: &FileFormat {
        id: 2_620_857_507,
        source_type: SourceType::Httpd,
        name: "matroska",
        extensions: &["mkv", "mk3d", "mks"],
        media_types: &["video/x-matroska"],
        signatures: &[],
        related_formats: &[],
    },
};
