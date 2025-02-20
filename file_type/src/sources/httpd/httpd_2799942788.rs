use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2799942788: FileType = FileType {
    file_format: &FileFormat {
        id: 2_799_942_788,
        source_type: SourceType::Httpd,
        name: "dvb subtitle",
        extensions: &["sub"],
        media_types: &["image/vnd.dvb.subtitle"],
        signatures: &[],
        related_formats: &[],
    },
};
