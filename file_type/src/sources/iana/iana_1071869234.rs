use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1071869234: FileFormat = FileFormat {
    id: 1_071_869_234,
    source_type: SourceType::Iana,
    name: "relax-ng-compact-syntax",
    extensions: &[],
    media_types: &["application/relax-ng-compact-syntax"],
    signatures: &[],
    related_formats: &[],
};
