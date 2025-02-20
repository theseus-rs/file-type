use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_929475271: FileType = FileType {
    file_format: &FileFormat {
        id: 929_475_271,
        source_type: SourceType::Httpd,
        name: "sun xml impress",
        extensions: &["sxi"],
        media_types: &["application/vnd.sun.xml.impress"],
        signatures: &[],
        related_formats: &[],
    },
};
