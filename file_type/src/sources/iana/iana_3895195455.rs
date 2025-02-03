use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3895195455: FileFormat = FileFormat {
    id: 3_895_195_455,
    source_type: SourceType::Iana,
    name: "vnd.afpc.modca-overlay",
    extensions: &[],
    media_types: &["application/vnd.afpc.modca-overlay"],
    internal_signatures: &[],
    related_formats: &[],
};
