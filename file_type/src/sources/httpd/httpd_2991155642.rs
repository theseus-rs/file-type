use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2991155642: FileFormat = FileFormat {
    id: 2_991_155_642,
    source_type: SourceType::Httpd,
    name: "ipunplugged rcprofile",
    extensions: &["rcprofile"],
    media_types: &["application/vnd.ipunplugged.rcprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
