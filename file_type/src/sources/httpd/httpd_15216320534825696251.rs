use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15216320534825696251: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hp hpgl",
    extensions: &["hpgl"],
    media_types: &["application/vnd.hp-hpgl"],
    internal_signatures: &[],
    related_formats: &[],
};
