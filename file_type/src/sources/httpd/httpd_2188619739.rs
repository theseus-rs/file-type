use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2188619739: FileType = FileType {
    file_format: &FileFormat {
        id: 2_188_619_739,
        source_type: SourceType::Httpd,
        name: "noblenet web",
        extensions: &["nnw"],
        media_types: &["application/vnd.noblenet-web"],
        signatures: &[],
        related_formats: &[],
    },
};
