use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4072025194511321041: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "medcalcdata",
    extensions: &["mc1"],
    media_types: &["application/vnd.medcalcdata"],
    internal_signatures: &[],
    related_formats: &[],
};
