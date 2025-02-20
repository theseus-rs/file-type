use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59693916: FileType = FileType {
    file_format: &FileFormat {
        id: 59_693_916,
        source_type: SourceType::Wikidata,
        name: "QuadriSpace format",
        extensions: &["qsd", "qsl", "qsm", "qst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
