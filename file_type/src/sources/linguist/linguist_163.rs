use crate::format::FileFormat;

pub(crate) const LINGUIST_163: FileFormat = FileFormat {
    id: 163,
    puid: "linguist/163",
    name: "INI",
    extensions: &[
        "cfg",
        "cnf",
        "dof",
        "ini",
        "lektorproject",
        "prefs",
        "pro",
        "properties",
        "url",
    ],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
