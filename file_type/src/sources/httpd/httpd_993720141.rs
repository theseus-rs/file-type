use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_993720141: FileType = FileType {
    file_format: &FileFormat {
        id: 993_720_141,
        source_type: SourceType::Httpd,
        name: "dvb subtitle",
        extensions: &["sub"],
        media_types: &["text/vnd.dvb.subtitle"],
        signatures: &[],
        related_formats: &[],
    },
};
