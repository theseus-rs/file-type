use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15363840241960418613: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kde kpresenter",
    extensions: &["kpr", "kpt"],
    media_types: &["application/vnd.kde.kpresenter"],
    internal_signatures: &[],
    related_formats: &[],
};
