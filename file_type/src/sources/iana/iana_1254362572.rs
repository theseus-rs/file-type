use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1254362572: FileFormat = FileFormat {
    id: 1_254_362_572,
    source_type: SourceType::Iana,
    name: "vnd.dtg.local.html",
    extensions: &[],
    media_types: &["application/vnd.dtg.local.html"],
    internal_signatures: &[],
    related_formats: &[],
};
