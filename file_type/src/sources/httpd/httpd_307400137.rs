use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_307400137: FileType = FileType {
    file_format: &FileFormat {
        id: 307_400_137,
        source_type: SourceType::Httpd,
        name: "igloader",
        extensions: &["igl"],
        media_types: &["application/vnd.igloader"],
        signatures: &[],
        related_formats: &[],
    },
};
