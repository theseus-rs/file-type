use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_224: FileFormat = FileFormat {
    id: 224,
    source_type: SourceType::Linguist,
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
