use crate::format::FileFormat;

pub(crate) const LINGUIST_332: FileFormat = FileFormat {
    id: 332,
    puid: "linguist/332",
    name: "SQF",
    extensions: &["hqf", "sqf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
