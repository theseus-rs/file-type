use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134395037: FileType = FileType {
    file_format: &FileFormat {
        id: 134_395_037,
        source_type: SourceType::Wikidata,
        name: "Maker Project file",
        extensions: &["mkr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
