use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_147: FileFormat = FileFormat {
    id: 147,
    source_type: SourceType::Linguist,
    name: "Jinja",
    extensions: &["j2", "jinja", "jinja2"],
    media_types: &["text/x-django"],
    internal_signatures: &[],
    related_formats: &[],
};
