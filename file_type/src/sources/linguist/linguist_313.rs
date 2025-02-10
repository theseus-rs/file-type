use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_313: FileType = FileType {
    file_format: &FileFormat {
        id: 313,
        source_type: SourceType::Linguist,
        name: "RMarkdown",
        extensions: &["qmd", "rmd"],
        media_types: &["text/x-gfm"],
        signatures: &[],
        related_formats: &[],
    },
};
