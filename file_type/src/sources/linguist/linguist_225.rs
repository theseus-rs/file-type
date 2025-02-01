use crate::format::FileFormat;

pub(crate) const LINGUIST_225: FileFormat = FileFormat {
    id: 225,
    puid: "linguist/225",
    name: "MATLAB",
    extensions: &["m", "matlab"],
    media_types: &["text/x-octave"],
    internal_signatures: &[],
    related_formats: &[],
};
