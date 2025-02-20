use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3003002017: FileType = FileType {
    file_format: &FileFormat {
        id: 3_003_002_017,
        source_type: SourceType::Httpd,
        name: "solent sdkm xml",
        extensions: &["sdkm", "sdkd"],
        media_types: &["application/vnd.solent.sdkm+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
