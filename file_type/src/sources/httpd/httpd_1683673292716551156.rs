use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1683673292716551156: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "las las xml",
    extensions: &["lasxml"],
    media_types: &["application/vnd.las.las+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
