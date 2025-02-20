use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_149374338: FileType = FileType {
    file_format: &FileFormat {
        id: 149_374_338,
        source_type: SourceType::Httpd,
        name: "portable bitmap",
        extensions: &["pbm"],
        media_types: &["image/x-portable-bitmap"],
        signatures: &[],
        related_formats: &[],
    },
};
