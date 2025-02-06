use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2576353381: FileFormat = FileFormat {
    id: 2_576_353_381,
    source_type: SourceType::Iana,
    name: "vnd.gnu.taler.exchange+json",
    extensions: &[],
    media_types: &["application/vnd.gnu.taler.exchange+json"],
    signatures: &[],
    related_formats: &[],
};
