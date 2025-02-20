use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356275: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_275,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif ES 'waves' format",
        extensions: &["w7w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
