use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2803882134: FileType = FileType {
    file_format: &FileFormat {
        id: 2_803_882_134,
        source_type: SourceType::Httpd,
        name: "hbci",
        extensions: &["hbci"],
        media_types: &["application/vnd.hbci"],
        signatures: &[],
        related_formats: &[],
    },
};
