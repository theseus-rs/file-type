use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3076879620: FileFormat = FileFormat {
    id: 3_076_879_620,
    source_type: SourceType::Httpd,
    name: "kde kword",
    extensions: &["kwd", "kwt"],
    media_types: &["application/vnd.kde.kword"],
    signatures: &[],
    related_formats: &[],
};
