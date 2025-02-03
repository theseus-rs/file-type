use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4254515506: FileFormat = FileFormat {
    id: 4_254_515_506,
    source_type: SourceType::Iana,
    name: "vnd.oftn.l10n+json",
    extensions: &[],
    media_types: &["application/vnd.oftn.l10n+json"],
    internal_signatures: &[],
    related_formats: &[],
};
