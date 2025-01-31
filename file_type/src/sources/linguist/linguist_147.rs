use crate::format::FileFormat;

pub(crate) const LINGUIST_147: FileFormat = FileFormat {
    id: 147,
    puid: "linguist/147",
    name: "Jinja",
    extensions: &["j2", "jinja", "jinja2"],
    media_types: &["text/x-django"],
    internal_signatures: &[],
    related_formats: &[],
};
