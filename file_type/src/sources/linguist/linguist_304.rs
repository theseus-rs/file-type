use crate::format::FileFormat;

pub(crate) const LINGUIST_304: FileFormat = FileFormat {
    id: 304,
    puid: "linguist/304",
    name: "Python traceback",
    extensions: &["pytb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
