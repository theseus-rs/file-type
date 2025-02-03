use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2985677434: FileFormat = FileFormat {
    id: 2_985_677_434,
    source_type: SourceType::Httpd,
    name: "audiograph",
    extensions: &["aep"],
    media_types: &["application/vnd.audiograph"],
    internal_signatures: &[],
    related_formats: &[],
};
