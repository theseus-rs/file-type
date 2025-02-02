use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7428950758947074420: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "publishare delta tree",
    extensions: &["qps"],
    media_types: &["application/vnd.publishare-delta-tree"],
    internal_signatures: &[],
    related_formats: &[],
};
