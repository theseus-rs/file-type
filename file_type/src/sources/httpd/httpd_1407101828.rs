use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1407101828: FileFormat = FileFormat {
    id: 1_407_101_828,
    source_type: SourceType::Httpd,
    name: "dece data",
    extensions: &["uvf", "uvvf", "uvd", "uvvd"],
    media_types: &["application/vnd.dece.data"],
    signatures: &[],
    related_formats: &[],
};
