use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83370740: FileType = FileType {
    file_format: &FileFormat {
        id: 83_370_740,
        source_type: SourceType::Wikidata,
        name: "Audio Visual Research",
        extensions: &["avr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
