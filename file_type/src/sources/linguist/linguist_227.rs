use crate::format::FileFormat;

pub(crate) const LINGUIST_227: FileFormat = FileFormat {
    id: 227,
    puid: "linguist/227",
    name: "Max",
    extensions: &["maxhelp", "maxpat", "maxproj", "mxt", "pat"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
