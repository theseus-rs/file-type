use crate::format::FileFormat;

pub(crate) const LINGUIST_315: FileFormat = FileFormat {
    id: 315,
    puid: "linguist/315",
    name: "RUNOFF",
    extensions: &["rnh", "rno"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
