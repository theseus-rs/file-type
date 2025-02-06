use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_835573590: FileFormat = FileFormat {
    id: 835_573_590,
    source_type: SourceType::Httpd,
    name: "yamaha hv voice",
    extensions: &["hvp"],
    media_types: &["application/vnd.yamaha.hv-voice"],
    signatures: &[],
    related_formats: &[],
};
