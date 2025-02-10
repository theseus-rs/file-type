use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_835573590: FileType = FileType {
    file_format: &FileFormat {
        id: 835_573_590,
        source_type: SourceType::Httpd,
        name: "yamaha hv voice",
        extensions: &["hvp"],
        media_types: &["application/vnd.yamaha.hv-voice"],
        signatures: &[],
        related_formats: &[],
    },
};
