use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2549361154: FileFormat = FileFormat {
    id: 2_549_361_154,
    source_type: SourceType::Httpd,
    name: "authorware seg",
    extensions: &["aas"],
    media_types: &["application/x-authorware-seg"],
    internal_signatures: &[],
    related_formats: &[],
};
