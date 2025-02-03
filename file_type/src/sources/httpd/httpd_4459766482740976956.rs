use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4459766482740976956: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fdsn mseed",
    extensions: &["mseed"],
    media_types: &["application/vnd.fdsn.mseed"],
    internal_signatures: &[],
    related_formats: &[],
};
