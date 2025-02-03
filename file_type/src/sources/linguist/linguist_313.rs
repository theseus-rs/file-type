use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_313: FileFormat = FileFormat {
    id: 313,
    source_type: SourceType::Linguist,
    name: "RMarkdown",
    extensions: &["qmd", "rmd"],
    media_types: &["text/x-gfm"],
    internal_signatures: &[],
    related_formats: &[],
};
