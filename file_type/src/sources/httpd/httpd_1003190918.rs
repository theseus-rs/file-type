use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1003190918: FileFormat = FileFormat {
    id: 1_003_190_918,
    source_type: SourceType::Httpd,
    name: "medcalcdata",
    extensions: &["mc1"],
    media_types: &["application/vnd.medcalcdata"],
    signatures: &[],
    related_formats: &[],
};
