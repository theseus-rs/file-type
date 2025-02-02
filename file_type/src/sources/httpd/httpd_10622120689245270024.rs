use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10622120689245270024: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kde kspread",
    extensions: &["ksp"],
    media_types: &["application/vnd.kde.kspread"],
    internal_signatures: &[],
    related_formats: &[],
};
