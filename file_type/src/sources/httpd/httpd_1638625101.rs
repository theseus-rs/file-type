use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1638625101: FileFormat = FileFormat {
    id: 1_638_625_101,
    source_type: SourceType::Httpd,
    name: "netcdf",
    extensions: &["nc", "cdf"],
    media_types: &["application/x-netcdf"],
    internal_signatures: &[],
    related_formats: &[],
};
