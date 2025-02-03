use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10354148540362929487: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "curl car",
    extensions: &["car"],
    media_types: &["application/vnd.curl.car"],
    internal_signatures: &[],
    related_formats: &[],
};
