use crate::format::FileFormat;

pub(crate) const LINGUIST_128: FileFormat = FileFormat {
    id: 128,
    puid: "linguist/128",
    name: "Gentoo Eclass",
    extensions: &["eclass"],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
