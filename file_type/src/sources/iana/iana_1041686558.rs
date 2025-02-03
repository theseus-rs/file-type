use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1041686558: FileFormat = FileFormat {
    id: 1_041_686_558,
    source_type: SourceType::Iana,
    name: "webpush-options+json",
    extensions: &[],
    media_types: &["application/webpush-options+json"],
    internal_signatures: &[],
    related_formats: &[],
};
