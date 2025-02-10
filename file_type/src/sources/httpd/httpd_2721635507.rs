use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2721635507: FileType = FileType {
    file_format: &FileFormat {
        id: 2_721_635_507,
        source_type: SourceType::Httpd,
        name: "micrografx igx",
        extensions: &["igx"],
        media_types: &["application/vnd.micrografx.igx"],
        signatures: &[],
        related_formats: &[],
    },
};
