use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1335481394: FileFormat = FileFormat {
    id: 1_335_481_394,
    source_type: SourceType::Httpd,
    name: "noblenet directory",
    extensions: &["nnd"],
    media_types: &["application/vnd.noblenet-directory"],
    internal_signatures: &[],
    related_formats: &[],
};
