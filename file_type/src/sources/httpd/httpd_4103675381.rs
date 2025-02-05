use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4103675381: FileFormat = FileFormat {
    id: 4_103_675_381,
    source_type: SourceType::Httpd,
    name: "epson salt",
    extensions: &["slt"],
    media_types: &["application/vnd.epson.salt"],
    signatures: &[],
    related_formats: &[],
};
