use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3683296436: FileType = FileType {
    file_format: &FileFormat {
        id: 3_683_296_436,
        source_type: SourceType::Httpd,
        name: "adobe formscentral fcdt",
        extensions: &["fcdt"],
        media_types: &["application/vnd.adobe.formscentral.fcdt"],
        signatures: &[],
        related_formats: &[],
    },
};
