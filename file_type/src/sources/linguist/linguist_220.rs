use crate::format::FileFormat;

pub(crate) const LINGUIST_220: FileFormat = FileFormat {
    id: 220,
    puid: "linguist/220",
    name: "Makefile",
    extensions: &["d", "mak", "make", "makefile", "mk", "mkfile"],
    media_types: &["text/x-cmake"],
    internal_signatures: &[],
    related_formats: &[],
};
