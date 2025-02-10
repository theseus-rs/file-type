use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205649: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_649,
        source_type: SourceType::Wikidata,
        name: "AAI",
        extensions: &["aai"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
