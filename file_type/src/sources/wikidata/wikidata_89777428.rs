use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_89777428: FileType = FileType {
    file_format: &FileFormat {
        id: 89_777_428,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 7",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
