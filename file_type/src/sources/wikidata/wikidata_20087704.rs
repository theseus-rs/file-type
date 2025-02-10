use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_20087704: FileType = FileType {
    file_format: &FileFormat {
        id: 20_087_704,
        source_type: SourceType::Wikidata,
        name: "TST",
        extensions: &["tst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
