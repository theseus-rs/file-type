use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1349874601: FileType = FileType {
    file_format: &FileFormat {
        id: 1_349_874_601,
        source_type: SourceType::Httpd,
        name: "ecowin chart",
        extensions: &["mag"],
        media_types: &["application/vnd.ecowin.chart"],
        signatures: &[],
        related_formats: &[],
    },
};
