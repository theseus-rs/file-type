use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_823982: FileType = FileType {
    file_format: &FileFormat {
        id: 823_982,
        source_type: SourceType::Wikidata,
        name: "MathML",
        extensions: &["mml"],
        media_types: &[
            "application/mathml+xml",
            "application/mathml-content+xml",
            "application/mathml-presentation+xml",
            "math/mml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
