use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34748575: FileType = FileType {
    file_format: &FileFormat {
        id: 34_748_575,
        source_type: SourceType::Wikidata,
        name: "Thermo-Calc Database Format",
        extensions: &["tdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
