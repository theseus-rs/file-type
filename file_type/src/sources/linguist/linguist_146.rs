use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_146: FileFormat = FileFormat {
    id: 146,
    source_type: SourceType::Linguist,
    name: "HTML",
    extensions: &["hta", "htm", "html", "html.hl", "inc", "xht", "xhtml"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
