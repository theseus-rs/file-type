use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2469893521022682751: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "blueice multipass",
    extensions: &["mpm"],
    media_types: &["application/vnd.blueice.multipass"],
    internal_signatures: &[],
    related_formats: &[],
};
