use crate::format::FileFormat;

pub(crate) const LINGUIST_93: FileFormat = FileFormat {
    id: 93,
    puid: "linguist/93",
    name: "ECL",
    extensions: &["ecl", "eclxml"],
    media_types: &["text/x-ecl"],
    internal_signatures: &[],
    related_formats: &[],
};
