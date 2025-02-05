use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_213710802: FileFormat = FileFormat {
    id: 213_710_802,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcvideo-ue-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcvideo-ue-config+xml"],
    signatures: &[],
    related_formats: &[],
};
