use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_733753614: FileType = FileType {
    file_format: &FileFormat {
        id: 733_753_614,
        source_type: SourceType::Httpd,
        name: "trid tpt",
        extensions: &["tpt"],
        media_types: &["application/vnd.trid.tpt"],
        signatures: &[],
        related_formats: &[],
    },
};
