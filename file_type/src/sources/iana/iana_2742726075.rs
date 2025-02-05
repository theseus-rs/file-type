use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2742726075: FileFormat = FileFormat {
    id: 2_742_726_075,
    source_type: SourceType::Iana,
    name: "vnd.moml+xml",
    extensions: &[],
    media_types: &["model/vnd.moml+xml"],
    signatures: &[],
    related_formats: &[],
};
