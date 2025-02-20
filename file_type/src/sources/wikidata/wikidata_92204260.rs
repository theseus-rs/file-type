use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_92204260: FileType = FileType {
    file_format: &FileFormat {
        id: 92_204_260,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 2018",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
