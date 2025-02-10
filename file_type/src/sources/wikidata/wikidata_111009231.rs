use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009231: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_231,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Poster File format",
        extensions: &["sig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
