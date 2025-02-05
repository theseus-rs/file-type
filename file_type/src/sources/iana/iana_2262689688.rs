use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2262689688: FileFormat = FileFormat {
    id: 2_262_689_688,
    source_type: SourceType::Iana,
    name: "vq-rtcpxr",
    extensions: &[],
    media_types: &["application/vq-rtcpxr"],
    signatures: &[],
    related_formats: &[],
};
