use crate::format::FileFormat;

pub(crate) const LINGUIST_146: FileFormat = FileFormat {
    id: 146,
    puid: "linguist/146",
    name: "HTML",
    extensions: &["hta", "htm", "html", "html.hl", "inc", "xht", "xhtml"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
