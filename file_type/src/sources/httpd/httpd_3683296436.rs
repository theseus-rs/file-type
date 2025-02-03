use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3683296436: FileFormat = FileFormat {
    id: 3_683_296_436,
    source_type: SourceType::Httpd,
    name: "adobe formscentral fcdt",
    extensions: &["fcdt"],
    media_types: &["application/vnd.adobe.formscentral.fcdt"],
    internal_signatures: &[],
    related_formats: &[],
};
