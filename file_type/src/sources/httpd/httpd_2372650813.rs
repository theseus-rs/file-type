use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2372650813: FileType = FileType {
    file_format: &FileFormat {
        id: 2_372_650_813,
        source_type: SourceType::Httpd,
        name: "cdmi capability",
        extensions: &["cdmia"],
        media_types: &["application/cdmi-capability"],
        signatures: &[],
        related_formats: &[],
    },
};
