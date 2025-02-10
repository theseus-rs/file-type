use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_96642275: FileType = FileType {
    file_format: &FileFormat {
        id: 96_642_275,
        source_type: SourceType::Linguist,
        name: "B4X",
        extensions: &["bas"],
        media_types: &["text/x-vb"],
        signatures: &[],
        related_formats: &[],
    },
};
