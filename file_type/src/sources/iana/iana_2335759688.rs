use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2335759688: FileFormat = FileFormat {
    id: 2_335_759_688,
    source_type: SourceType::Iana,
    name: "vnd.veraison.tsm-report+json",
    extensions: &[],
    media_types: &["application/vnd.veraison.tsm-report+json"],
    internal_signatures: &[],
    related_formats: &[],
};
