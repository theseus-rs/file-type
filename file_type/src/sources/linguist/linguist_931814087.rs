use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_931814087: FileFormat = FileFormat {
    id: 931_814_087,
    source_type: SourceType::Linguist,
    name: "HiveQL",
    extensions: &["hql", "q"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
