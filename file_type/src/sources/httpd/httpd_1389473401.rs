use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1389473401: FileFormat = FileFormat {
    id: 1_389_473_401,
    source_type: SourceType::Httpd,
    name: "kde kpresenter",
    extensions: &["kpr", "kpt"],
    media_types: &["application/vnd.kde.kpresenter"],
    internal_signatures: &[],
    related_formats: &[],
};
