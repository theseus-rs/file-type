use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356290: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_290,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif ES sample data file",
        extensions: &["w8v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
