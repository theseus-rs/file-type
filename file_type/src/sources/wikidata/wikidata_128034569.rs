use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128034569: FileType = FileType {
    file_format: &FileFormat {
        id: 128_034_569,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 19",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
