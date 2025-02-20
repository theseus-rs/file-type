use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_477582706: FileType = FileType {
    file_format: &FileFormat {
        id: 477_582_706,
        source_type: SourceType::Linguist,
        name: "Motorola 68K Assembly",
        extensions: &["asm", "i", "inc", "s", "x68"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
