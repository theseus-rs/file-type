use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3638789432: FileType = FileType {
    file_format: &FileFormat {
        id: 3_638_789_432,
        source_type: SourceType::Httpd,
        name: "anser web funds transfer initiation",
        extensions: &["fti"],
        media_types: &["application/vnd.anser-web-funds-transfer-initiation"],
        signatures: &[],
        related_formats: &[],
    },
};
