use crate::format::FileFormat;

pub(crate) const LINGUIST_97: FileFormat = FileFormat {
    id: 97,
    puid: "linguist/97",
    name: "Eagle",
    extensions: &["brd", "sch"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
