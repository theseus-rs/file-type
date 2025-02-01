use crate::format::FileFormat;

pub(crate) const LINGUIST_307: FileFormat = FileFormat {
    id: 307,
    puid: "linguist/307",
    name: "R",
    extensions: &["r", "rd", "rsx"],
    media_types: &["text/x-rsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
