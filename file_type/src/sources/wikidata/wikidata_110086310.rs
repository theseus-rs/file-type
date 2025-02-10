use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110086310: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_310,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 2021",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
