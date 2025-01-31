use crate::format::FileFormat;

pub(crate) const LINGUIST_313: FileFormat = FileFormat {
    id: 313,
    puid: "linguist/313",
    name: "RMarkdown",
    extensions: &["qmd", "rmd"],
    media_types: &["text/x-gfm"],
    internal_signatures: &[],
    related_formats: &[],
};
