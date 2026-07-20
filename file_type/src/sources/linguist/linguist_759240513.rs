use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_759240513: FileType = FileType {
    file_format: &FileFormat {
        id: 759_240_513,
        source_type: SourceType::Linguist,
        name: "Lambdapi",
        extensions: &["lp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
