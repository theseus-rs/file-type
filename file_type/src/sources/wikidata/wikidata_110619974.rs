use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110619974: FileType = FileType {
    file_format: &FileFormat {
        id: 110_619_974,
        source_type: SourceType::Wikidata,
        name: "Adobe Atmosphere world (.atmo)",
        extensions: &["3da", "aer", "atmo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
