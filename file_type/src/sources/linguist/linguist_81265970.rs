use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_81265970: FileFormat = FileFormat {
    id: 81_265_970,
    source_type: SourceType::Linguist,
    name: "Vim Snippet",
    extensions: &["snip", "snippet", "snippets"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
