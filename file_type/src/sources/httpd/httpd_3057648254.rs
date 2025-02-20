use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3057648254: FileType = FileType {
    file_format: &FileFormat {
        id: 3_057_648_254,
        source_type: SourceType::Httpd,
        name: "contact cmsg",
        extensions: &["cdbcmsg"],
        media_types: &["application/vnd.contact.cmsg"],
        signatures: &[],
        related_formats: &[],
    },
};
