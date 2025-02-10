use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3759633579: FileType = FileType {
    file_format: &FileFormat {
        id: 3_759_633_579,
        source_type: SourceType::Httpd,
        name: "pkcs8",
        extensions: &["p8"],
        media_types: &["application/pkcs8"],
        signatures: &[],
        related_formats: &[],
    },
};
