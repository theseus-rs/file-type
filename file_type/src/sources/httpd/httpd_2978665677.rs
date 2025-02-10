use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2978665677: FileType = FileType {
    file_format: &FileFormat {
        id: 2_978_665_677,
        source_type: SourceType::Httpd,
        name: "claymore",
        extensions: &["cla"],
        media_types: &["application/vnd.claymore"],
        signatures: &[],
        related_formats: &[],
    },
};
