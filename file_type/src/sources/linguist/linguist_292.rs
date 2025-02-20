use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_292: FileType = FileType {
    file_format: &FileFormat {
        id: 292,
        source_type: SourceType::Linguist,
        name: "PowerBuilder",
        extensions: &["pbt", "sra", "sru", "srw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
