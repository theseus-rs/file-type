use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_994343837: FileFormat = FileFormat {
    id: 994_343_837,
    source_type: SourceType::Httpd,
    name: "rn realmedia",
    extensions: &["rm"],
    media_types: &["application/vnd.rn-realmedia"],
    internal_signatures: &[],
    related_formats: &[],
};
