use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3201493925: FileType = FileType {
    file_format: &FileFormat {
        id: 3_201_493_925,
        source_type: SourceType::Httpd,
        name: "pgp encrypted",
        extensions: &["pgp"],
        media_types: &["application/pgp-encrypted"],
        signatures: &[],
        related_formats: &[],
    },
};
