use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_880010326: FileFormat = FileFormat {
    id: 880_010_326,
    source_type: SourceType::Linguist,
    name: "SELinux Policy",
    extensions: &["te"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
