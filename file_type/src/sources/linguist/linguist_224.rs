use crate::format::FileFormat;

pub(crate) const LINGUIST_224: FileFormat = FileFormat {
    id: 224,
    puid: "linguist/224",
    name: "Mathematica",
    extensions: &[
        "cdf",
        "m",
        "ma",
        "mathematica",
        "mt",
        "nb",
        "nbp",
        "wl",
        "wlt",
    ],
    media_types: &["text/x-mathematica"],
    internal_signatures: &[],
    related_formats: &[],
};
