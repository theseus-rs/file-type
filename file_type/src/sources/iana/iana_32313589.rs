use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_32313589: FileFormat = FileFormat {
    id: 32_313_589,
    source_type: SourceType::Iana,
    name: "vnd.artisan+json",
    extensions: &[],
    media_types: &["application/vnd.artisan+json"],
    internal_signatures: &[],
    related_formats: &[],
};
