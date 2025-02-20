use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
