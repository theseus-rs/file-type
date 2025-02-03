use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2764967758: FileFormat = FileFormat {
    id: 2_764_967_758,
    source_type: SourceType::Httpd,
    name: "scvp cv request",
    extensions: &["scq"],
    media_types: &["application/scvp-cv-request"],
    internal_signatures: &[],
    related_formats: &[],
};
