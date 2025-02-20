use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27121524: FileType = FileType {
    file_format: &FileFormat {
        id: 27_121_524,
        source_type: SourceType::Wikidata,
        name: "fixed width text file",
        extensions: &["fwf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
