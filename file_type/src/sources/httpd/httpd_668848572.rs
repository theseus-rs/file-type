use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_668848572: FileType = FileType {
    file_format: &FileFormat {
        id: 668_848_572,
        source_type: SourceType::Httpd,
        name: "fujitsu oasys3",
        extensions: &["oa3"],
        media_types: &["application/vnd.fujitsu.oasys3"],
        signatures: &[],
        related_formats: &[],
    },
};
