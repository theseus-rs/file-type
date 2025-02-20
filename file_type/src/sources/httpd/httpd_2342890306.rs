use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2342890306: FileType = FileType {
    file_format: &FileFormat {
        id: 2_342_890_306,
        source_type: SourceType::Httpd,
        name: "picsel",
        extensions: &["efif"],
        media_types: &["application/vnd.picsel"],
        signatures: &[],
        related_formats: &[],
    },
};
