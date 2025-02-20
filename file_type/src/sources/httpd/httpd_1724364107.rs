use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1724364107: FileType = FileType {
    file_format: &FileFormat {
        id: 1_724_364_107,
        source_type: SourceType::Httpd,
        name: "hp pcl",
        extensions: &["pcl"],
        media_types: &["application/vnd.hp-pcl"],
        signatures: &[],
        related_formats: &[],
    },
};
