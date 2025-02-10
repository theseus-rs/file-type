use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1003190918: FileType = FileType {
    file_format: &FileFormat {
        id: 1_003_190_918,
        source_type: SourceType::Httpd,
        name: "medcalcdata",
        extensions: &["mc1"],
        media_types: &["application/vnd.medcalcdata"],
        signatures: &[],
        related_formats: &[],
    },
};
