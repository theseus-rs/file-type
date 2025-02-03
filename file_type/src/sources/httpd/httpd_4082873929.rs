use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4082873929: FileFormat = FileFormat {
    id: 4_082_873_929,
    source_type: SourceType::Httpd,
    name: "llamagraphics life balance exchange xml",
    extensions: &["lbe"],
    media_types: &["application/vnd.llamagraphics.life-balance.exchange+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
