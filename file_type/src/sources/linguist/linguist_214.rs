use crate::format::FileFormat;

pub(crate) const LINGUIST_214: FileFormat = FileFormat {
    id: 214,
    puid: "linguist/214",
    name: "M",
    extensions: &["m", "mumps"],
    media_types: &["text/x-mumps"],
    internal_signatures: &[],
    related_formats: &[],
};
