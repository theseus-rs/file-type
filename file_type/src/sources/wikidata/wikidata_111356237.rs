use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356237: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_237,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif (6/7/8) sample data file",
        extensions: &["w3v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
