use crate::format::FileFormat;

pub(crate) const LINGUIST_242: FileFormat = FileFormat {
    id: 242,
    puid: "linguist/242",
    name: "NSIS",
    extensions: &["nsh", "nsi"],
    media_types: &["text/x-nsis"],
    internal_signatures: &[],
    related_formats: &[],
};
