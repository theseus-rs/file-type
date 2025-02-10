use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3024262940: FileType = FileType {
    file_format: &FileFormat {
        id: 3_024_262_940,
        source_type: SourceType::Httpd,
        name: "geoplan",
        extensions: &["g2w"],
        media_types: &["application/vnd.geoplan"],
        signatures: &[],
        related_formats: &[],
    },
};
