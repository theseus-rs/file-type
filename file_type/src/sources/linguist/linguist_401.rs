use crate::format::FileFormat;

pub(crate) const LINGUIST_401: FileFormat = FileFormat {
    id: 401,
    puid: "linguist/401",
    name: "XProc",
    extensions: &["xpl", "xproc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
