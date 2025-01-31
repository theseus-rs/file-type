use crate::format::FileFormat;

pub(crate) const LINGUIST_105: FileFormat = FileFormat {
    id: 105,
    puid: "linguist/105",
    name: "F#",
    extensions: &["fs", "fsi", "fsx"],
    media_types: &["text/x-fsharp"],
    internal_signatures: &[],
    related_formats: &[],
};
