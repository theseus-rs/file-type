use crate::format::FileFormat;

pub(crate) const LINGUIST_358: FileFormat = FileFormat {
    id: 358,
    puid: "linguist/358",
    name: "Stata",
    extensions: &["ado", "do", "doh", "ihlp", "mata", "matah", "sthlp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
