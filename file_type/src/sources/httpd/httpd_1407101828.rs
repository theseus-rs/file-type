use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1407101828: FileType = FileType {
    file_format: &FileFormat {
        id: 1_407_101_828,
        source_type: SourceType::Httpd,
        name: "dece data",
        extensions: &["uvf", "uvvf", "uvd", "uvvd"],
        media_types: &["application/vnd.dece.data"],
        signatures: &[],
        related_formats: &[],
    },
};
