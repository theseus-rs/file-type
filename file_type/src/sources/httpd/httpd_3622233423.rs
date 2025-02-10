use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3622233423: FileType = FileType {
    file_format: &FileFormat {
        id: 3_622_233_423,
        source_type: SourceType::Httpd,
        name: "openofficeorg extension",
        extensions: &["oxt"],
        media_types: &["application/vnd.openofficeorg.extension"],
        signatures: &[],
        related_formats: &[],
    },
};
