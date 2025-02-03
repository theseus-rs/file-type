use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1255250193: FileFormat = FileFormat {
    id: 1_255_250_193,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-msgstore-ctrl-request+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
