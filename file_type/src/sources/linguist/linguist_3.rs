use crate::format::FileFormat;

pub(crate) const LINGUIST_3: FileFormat = FileFormat {
    id: 3,
    puid: "linguist/3",
    name: "AMPL",
    extensions: &["ampl", "mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
