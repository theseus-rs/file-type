use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2165814283: FileFormat = FileFormat {
    id: 2_165_814_283,
    source_type: SourceType::Httpd,
    name: "novadigm ext",
    extensions: &["ext"],
    media_types: &["application/vnd.novadigm.ext"],
    internal_signatures: &[],
    related_formats: &[],
};
