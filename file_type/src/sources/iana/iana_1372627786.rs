use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1372627786: FileFormat = FileFormat {
    id: 1_372_627_786,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcvideo-affiliation-command+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcvideo-affiliation-command+xml"],
    signatures: &[],
    related_formats: &[],
};
