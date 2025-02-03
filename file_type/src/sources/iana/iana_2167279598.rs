use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2167279598: FileFormat = FileFormat {
    id: 2_167_279_598,
    source_type: SourceType::Iana,
    name: "vnd.veritone.aion+json",
    extensions: &[],
    media_types: &["application/vnd.veritone.aion+json"],
    internal_signatures: &[],
    related_formats: &[],
};
