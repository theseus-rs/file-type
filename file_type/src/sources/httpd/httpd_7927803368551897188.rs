use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7927803368551897188: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "netcdf",
    extensions: &["nc", "cdf"],
    media_types: &["application/x-netcdf"],
    internal_signatures: &[],
    related_formats: &[],
};
