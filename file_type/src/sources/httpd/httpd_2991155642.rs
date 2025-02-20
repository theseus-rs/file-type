use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2991155642: FileType = FileType {
    file_format: &FileFormat {
        id: 2_991_155_642,
        source_type: SourceType::Httpd,
        name: "ipunplugged rcprofile",
        extensions: &["rcprofile"],
        media_types: &["application/vnd.ipunplugged.rcprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
