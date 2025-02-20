use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_795579337: FileType = FileType {
    file_format: &FileFormat {
        id: 795_579_337,
        source_type: SourceType::Linguist,
        name: "templ",
        extensions: &["templ"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
