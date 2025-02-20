use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127269093: FileType = FileType {
    file_format: &FileFormat {
        id: 127_269_093,
        source_type: SourceType::Wikidata,
        name: "Bulk Data File",
        extensions: &["bdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
