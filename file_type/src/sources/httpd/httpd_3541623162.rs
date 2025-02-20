use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3541623162: FileType = FileType {
    file_format: &FileFormat {
        id: 3_541_623_162,
        source_type: SourceType::Httpd,
        name: "mathematica",
        extensions: &["ma", "nb", "mb"],
        media_types: &["application/mathematica"],
        signatures: &[],
        related_formats: &[],
    },
};
