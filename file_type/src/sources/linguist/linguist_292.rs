use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_292: FileFormat = FileFormat {
    id: 292,
    source_type: SourceType::Linguist,
    name: "PowerBuilder",
    extensions: &["pbt", "sra", "sru", "srw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
