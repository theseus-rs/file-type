use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_835573590: FileType = FileType {
    file_format: &FileFormat {
        id: 835_573_590,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.hv-voice",
        extensions: &[],
        media_types: &["application/vnd.yamaha.hv-voice"],
        signatures: &[],
        related_formats: &[],
    },
};
