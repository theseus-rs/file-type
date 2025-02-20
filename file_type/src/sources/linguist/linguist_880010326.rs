use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_880010326: FileType = FileType {
    file_format: &FileFormat {
        id: 880_010_326,
        source_type: SourceType::Linguist,
        name: "SELinux Policy",
        extensions: &["te"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
