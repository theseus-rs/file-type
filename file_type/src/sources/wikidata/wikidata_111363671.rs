use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363671: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_671,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif XS 'all' format",
        extensions: &["x0a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
