use crate::format::FileFormat;

pub(crate) const LINGUIST_63: FileFormat = FileFormat {
    id: 63,
    puid: "linguist/63",
    name: "CoffeeScript",
    extensions: &["_coffee", "cake", "cjsx", "coffee", "iced"],
    media_types: &["text/x-coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};
