use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3629759553: FileType = FileType {
    file_format: &FileFormat {
        id: 3_629_759_553,
        source_type: SourceType::Httpd,
        name: "dvi",
        extensions: &["dvi"],
        media_types: &["application/x-dvi"],
        signatures: &[],
        related_formats: &[],
    },
};
