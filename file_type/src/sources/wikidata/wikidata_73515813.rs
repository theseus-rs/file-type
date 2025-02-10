use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73515813: FileType = FileType {
    file_format: &FileFormat {
        id: 73_515_813,
        source_type: SourceType::Wikidata,
        name: "Palantir WinTime Plan",
        extensions: &["pln"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
