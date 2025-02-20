use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4012838875: FileType = FileType {
    file_format: &FileFormat {
        id: 4_012_838_875,
        source_type: SourceType::Httpd,
        name: "denovo fcselayout link",
        extensions: &["fe_launch"],
        media_types: &["application/vnd.denovo.fcselayout-link"],
        signatures: &[],
        related_formats: &[],
    },
};
