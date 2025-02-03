use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10582393778901387068: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mfmp",
    extensions: &["mfm"],
    media_types: &["application/vnd.mfmp"],
    internal_signatures: &[],
    related_formats: &[],
};
