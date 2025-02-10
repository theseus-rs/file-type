use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_981279644: FileType = FileType {
    file_format: &FileFormat {
        id: 981_279_644,
        source_type: SourceType::Httpd,
        name: "dvb ait",
        extensions: &["ait"],
        media_types: &["application/vnd.dvb.ait"],
        signatures: &[],
        related_formats: &[],
    },
};
