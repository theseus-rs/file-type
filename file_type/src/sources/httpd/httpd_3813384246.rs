use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3813384246: FileFormat = FileFormat {
    id: 3_813_384_246,
    source_type: SourceType::Httpd,
    name: "aristanetworks swi",
    extensions: &["swi"],
    media_types: &["application/vnd.aristanetworks.swi"],
    internal_signatures: &[],
    related_formats: &[],
};
