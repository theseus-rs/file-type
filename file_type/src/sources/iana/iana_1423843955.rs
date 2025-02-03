use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1423843955: FileFormat = FileFormat {
    id: 1_423_843_955,
    source_type: SourceType::Iana,
    name: "alto-tipsparams+json",
    extensions: &[],
    media_types: &["application/alto-tipsparams+json"],
    internal_signatures: &[],
    related_formats: &[],
};
