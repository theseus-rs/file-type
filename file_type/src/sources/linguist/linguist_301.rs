use crate::format::FileFormat;

pub(crate) const LINGUIST_301: FileFormat = FileFormat {
    id: 301,
    puid: "linguist/301",
    name: "PureBasic",
    extensions: &["pb", "pbi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
