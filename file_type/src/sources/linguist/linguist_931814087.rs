use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_931814087: FileType = FileType {
    file_format: &FileFormat {
        id: 931_814_087,
        source_type: SourceType::Linguist,
        name: "HiveQL",
        extensions: &["hql", "q"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
