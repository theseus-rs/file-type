use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_348215381: FileFormat = FileFormat {
    id: 348_215_381,
    source_type: SourceType::Httpd,
    name: "kde kchart",
    extensions: &["chrt"],
    media_types: &["application/vnd.kde.kchart"],
    signatures: &[],
    related_formats: &[],
};
