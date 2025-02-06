use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_565406949: FileFormat = FileFormat {
    id: 565_406_949,
    source_type: SourceType::Httpd,
    name: "intergeo",
    extensions: &["i2g"],
    media_types: &["application/vnd.intergeo"],
    signatures: &[],
    related_formats: &[],
};
