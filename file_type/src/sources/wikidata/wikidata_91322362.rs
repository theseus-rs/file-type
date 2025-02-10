use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_91322362: FileType = FileType {
    file_format: &FileFormat {
        id: 91_322_362,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 2017",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
