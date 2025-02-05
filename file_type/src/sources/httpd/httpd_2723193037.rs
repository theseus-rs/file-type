use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2723193037: FileFormat = FileFormat {
    id: 2_723_193_037,
    source_type: SourceType::Httpd,
    name: "kde kformula",
    extensions: &["kfo"],
    media_types: &["application/vnd.kde.kformula"],
    signatures: &[],
    related_formats: &[],
};
