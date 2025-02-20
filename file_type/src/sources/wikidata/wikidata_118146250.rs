use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118146250: FileType = FileType {
    file_format: &FileFormat {
        id: 118_146_250,
        source_type: SourceType::Wikidata,
        name: "Stripline File",
        extensions: &["tl4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
