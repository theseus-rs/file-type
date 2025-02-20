use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356268: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_268,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif ES 'voices' format",
        extensions: &["w7v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
