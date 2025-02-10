use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_996033646: FileType = FileType {
    file_format: &FileFormat {
        id: 996_033_646,
        source_type: SourceType::Httpd,
        name: "mseq",
        extensions: &["mseq"],
        media_types: &["application/vnd.mseq"],
        signatures: &[],
        related_formats: &[],
    },
};
