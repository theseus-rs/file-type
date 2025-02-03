use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4012838875: FileFormat = FileFormat {
    id: 4_012_838_875,
    source_type: SourceType::Httpd,
    name: "denovo fcselayout link",
    extensions: &["fe_launch"],
    media_types: &["application/vnd.denovo.fcselayout-link"],
    internal_signatures: &[],
    related_formats: &[],
};
