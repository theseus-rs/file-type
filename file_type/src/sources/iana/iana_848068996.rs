use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_848068996: FileFormat = FileFormat {
    id: 848_068_996,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcvideo-service-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcvideo-service-config+xml"],
    signatures: &[],
    related_formats: &[],
};
