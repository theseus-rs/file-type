use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4293499535: FileType = FileType {
    file_format: &FileFormat {
        id: 4_293_499_535,
        source_type: SourceType::Httpd,
        name: "dvb service",
        extensions: &["svc"],
        media_types: &["application/vnd.dvb.service"],
        signatures: &[],
        related_formats: &[],
    },
};
