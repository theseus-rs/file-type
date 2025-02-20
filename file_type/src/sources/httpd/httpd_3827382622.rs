use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3827382622: FileType = FileType {
    file_format: &FileFormat {
        id: 3_827_382_622,
        source_type: SourceType::Httpd,
        name: "mac binhex40",
        extensions: &["hqx"],
        media_types: &["application/mac-binhex40"],
        signatures: &[],
        related_formats: &[],
    },
};
