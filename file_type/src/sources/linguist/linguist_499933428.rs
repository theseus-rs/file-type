use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_499933428: FileType = FileType {
    file_format: &FileFormat {
        id: 499_933_428,
        source_type: SourceType::Linguist,
        name: "Prisma",
        extensions: &["prisma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
