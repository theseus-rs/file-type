use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4179771948179943531: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "criticaltools wbs xml",
    extensions: &["wbs"],
    media_types: &["application/vnd.criticaltools.wbs+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
