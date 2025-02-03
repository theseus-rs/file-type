use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_566198445: FileFormat = FileFormat {
    id: 566_198_445,
    source_type: SourceType::Linguist,
    name: "mdsvex",
    extensions: &["svx"],
    media_types: &["text/x-gfm"],
    internal_signatures: &[],
    related_formats: &[],
};
