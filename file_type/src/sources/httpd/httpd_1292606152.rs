use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1292606152: FileFormat = FileFormat {
    id: 1_292_606_152,
    source_type: SourceType::Httpd,
    name: "texinfo",
    extensions: &["texinfo", "texi"],
    media_types: &["application/x-texinfo"],
    signatures: &[],
    related_formats: &[],
};
