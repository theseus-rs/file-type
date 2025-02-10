use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113376365: FileType = FileType {
    file_format: &FileFormat {
        id: 113_376_365,
        source_type: SourceType::Wikidata,
        name: "Yamaha Wave Audio Generic",
        extensions: &["f01", "s01", "u01", "w01"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
