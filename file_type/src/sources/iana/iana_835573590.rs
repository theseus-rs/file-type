use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_835573590: FileFormat = FileFormat {
    id: 835_573_590,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.hv-voice",
    extensions: &[],
    media_types: &["application/vnd.yamaha.hv-voice"],
    signatures: &[],
    related_formats: &[],
};
