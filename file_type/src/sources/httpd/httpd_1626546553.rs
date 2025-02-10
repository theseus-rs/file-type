use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1626546553: FileType = FileType {
    file_format: &FileFormat {
        id: 1_626_546_553,
        source_type: SourceType::Httpd,
        name: "isac fcs",
        extensions: &["fcs"],
        media_types: &["application/vnd.isac.fcs"],
        signatures: &[],
        related_formats: &[],
    },
};
