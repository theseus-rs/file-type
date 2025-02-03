use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_863031821: FileFormat = FileFormat {
    id: 863_031_821,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.gtpc",
    extensions: &[],
    media_types: &["application/vnd.3gpp.gtpc"],
    internal_signatures: &[],
    related_formats: &[],
};
