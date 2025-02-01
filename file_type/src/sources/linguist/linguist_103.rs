use crate::format::FileFormat;

pub(crate) const LINGUIST_103: FileFormat = FileFormat {
    id: 103,
    puid: "linguist/103",
    name: "EmberScript",
    extensions: &["em", "emberscript"],
    media_types: &["text/x-coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};
