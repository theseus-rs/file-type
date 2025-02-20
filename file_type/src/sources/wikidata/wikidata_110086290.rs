use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110086290: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_290,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 2020",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
