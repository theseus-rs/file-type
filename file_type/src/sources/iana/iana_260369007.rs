use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_260369007: FileFormat = FileFormat {
    id: 260_369_007,
    source_type: SourceType::Iana,
    name: "sslkeylogfile",
    extensions: &[],
    media_types: &["application/sslkeylogfile"],
    internal_signatures: &[],
    related_formats: &[],
};
