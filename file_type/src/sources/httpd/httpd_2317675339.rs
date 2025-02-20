use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2317675339: FileType = FileType {
    file_format: &FileFormat {
        id: 2_317_675_339,
        source_type: SourceType::Httpd,
        name: "oasis opendocument chart template",
        extensions: &["otc"],
        media_types: &["application/vnd.oasis.opendocument.chart-template"],
        signatures: &[],
        related_formats: &[],
    },
};
