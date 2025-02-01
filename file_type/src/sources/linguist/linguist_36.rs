use crate::format::FileFormat;

pub(crate) const LINGUIST_36: FileFormat = FileFormat {
    id: 36,
    puid: "linguist/36",
    name: "Bluespec",
    extensions: &["bsv"],
    media_types: &["text/x-systemverilog"],
    internal_signatures: &[],
    related_formats: &[],
};
