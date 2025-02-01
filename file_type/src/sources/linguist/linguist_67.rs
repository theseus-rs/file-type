use crate::format::FileFormat;

pub(crate) const LINGUIST_67: FileFormat = FileFormat {
    id: 67,
    puid: "linguist/67",
    name: "Component Pascal",
    extensions: &["cp", "cps"],
    media_types: &["text/x-pascal"],
    internal_signatures: &[],
    related_formats: &[],
};
