use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2306433952: FileFormat = FileFormat {
    id: 2_306_433_952,
    source_type: SourceType::Httpd,
    name: "mobius dis",
    extensions: &["dis"],
    media_types: &["application/vnd.mobius.dis"],
    internal_signatures: &[],
    related_formats: &[],
};
