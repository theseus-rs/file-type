use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4291861971: FileType = FileType {
    file_format: &FileFormat {
        id: 4_291_861_971,
        source_type: SourceType::Httpd,
        name: "cups ppd",
        extensions: &["ppd"],
        media_types: &["application/vnd.cups-ppd"],
        signatures: &[],
        related_formats: &[],
    },
};
