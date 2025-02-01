use crate::format::FileFormat;

pub(crate) const LINGUIST_127: FileFormat = FileFormat {
    id: 127,
    puid: "linguist/127",
    name: "Gentoo Ebuild",
    extensions: &["ebuild"],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
