use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2803882134: FileFormat = FileFormat {
    id: 2_803_882_134,
    source_type: SourceType::Httpd,
    name: "hbci",
    extensions: &["hbci"],
    media_types: &["application/vnd.hbci"],
    signatures: &[],
    related_formats: &[],
};
