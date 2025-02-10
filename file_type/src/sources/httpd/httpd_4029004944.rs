use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4029004944: FileType = FileType {
    file_format: &FileFormat {
        id: 4_029_004_944,
        source_type: SourceType::Httpd,
        name: "xml",
        extensions: &["xml", "xsl"],
        media_types: &["application/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
