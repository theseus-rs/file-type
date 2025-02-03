use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4029004944: FileFormat = FileFormat {
    id: 4_029_004_944,
    source_type: SourceType::Httpd,
    name: "xml",
    extensions: &["xml", "xsl"],
    media_types: &["application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
