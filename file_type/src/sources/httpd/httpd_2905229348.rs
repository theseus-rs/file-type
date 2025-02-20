use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2905229348: FileType = FileType {
    file_format: &FileFormat {
        id: 2_905_229_348,
        source_type: SourceType::Httpd,
        name: "oasis opendocument chart",
        extensions: &["odc"],
        media_types: &["application/vnd.oasis.opendocument.chart"],
        signatures: &[],
        related_formats: &[],
    },
};
