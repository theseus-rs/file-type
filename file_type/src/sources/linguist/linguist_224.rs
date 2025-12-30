use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_224: FileType = FileType {
    file_format: &FileFormat {
        id: 224,
        source_type: SourceType::Linguist,
        name: "Wolfram Language",
        extensions: &[
            "cdf",
            "m",
            "ma",
            "mathematica",
            "mt",
            "nb",
            "nbp",
            "wl",
            "wls",
            "wlt",
        ],
        media_types: &["text/x-mathematica"],
        signatures: &[],
        related_formats: &[],
    },
};
