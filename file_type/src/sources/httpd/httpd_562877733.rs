use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_562877733: FileType = FileType {
    file_format: &FileFormat {
        id: 562_877_733,
        source_type: SourceType::Httpd,
        name: "rn realmedia vbr",
        extensions: &["rmvb"],
        media_types: &["application/vnd.rn-realmedia-vbr"],
        signatures: &[],
        related_formats: &[],
    },
};
