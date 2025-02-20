use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1692368822: FileType = FileType {
    file_format: &FileFormat {
        id: 1_692_368_822,
        source_type: SourceType::Httpd,
        name: "genomatix tuxedo",
        extensions: &["txd"],
        media_types: &["application/vnd.genomatix.tuxedo"],
        signatures: &[],
        related_formats: &[],
    },
};
