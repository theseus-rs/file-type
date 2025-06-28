use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1067292664: FileType = FileType {
    file_format: &FileFormat {
        id: 1_067_292_664,
        source_type: SourceType::Linguist,
        name: "Nickel",
        extensions: &["ncl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
