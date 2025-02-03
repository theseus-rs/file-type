use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4019441519: FileFormat = FileFormat {
    id: 4_019_441_519,
    source_type: SourceType::Httpd,
    name: "airzip filesecure azs",
    extensions: &["azs"],
    media_types: &["application/vnd.airzip.filesecure.azs"],
    internal_signatures: &[],
    related_formats: &[],
};
