use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_22565941: FileFormat = FileFormat {
    id: 22_565_941,
    source_type: SourceType::Iana,
    name: "vnd.oma.scidm.messages+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.scidm.messages+xml"],
    signatures: &[],
    related_formats: &[],
};
