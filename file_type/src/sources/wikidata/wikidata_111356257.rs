use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356257: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_257,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif ES 'all' format",
        extensions: &["w7a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
