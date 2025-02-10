use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_224: FileType = FileType {
    file_format: &FileFormat {
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
        signatures: &[],
        related_formats: &[],
    },
};
