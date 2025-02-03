use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2189723454: FileFormat = FileFormat {
    id: 2_189_723_454,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-floor-request+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-floor-request+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
