use crate::format::FileFormat;

pub(crate) const LINGUIST_149: FileFormat = FileFormat {
    id: 149,
    puid: "linguist/149",
    name: "HTML+EEX",
    extensions: &["heex", "html.eex", "leex"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
