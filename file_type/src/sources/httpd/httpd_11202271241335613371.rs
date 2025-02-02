use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11202271241335613371: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cfs compressed",
    extensions: &["cfs"],
    media_types: &["application/x-cfs-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
