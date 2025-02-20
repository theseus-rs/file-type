use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_906694254: FileType = FileType {
    file_format: &FileFormat {
        id: 906_694_254,
        source_type: SourceType::Linguist,
        name: "FIRRTL",
        extensions: &["fir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
