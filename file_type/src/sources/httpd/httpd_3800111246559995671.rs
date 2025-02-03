use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3800111246559995671: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "scvp vp response",
    extensions: &["spp"],
    media_types: &["application/scvp-vp-response"],
    internal_signatures: &[],
    related_formats: &[],
};
