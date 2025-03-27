use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113661597: FileType = FileType {
    file_format: &FileFormat {
        id: 113_661_597,
        source_type: SourceType::Wikidata,
        name: "Casio RAW Image",
        extensions: &["bay"],
        media_types: &["image/x-raw-casio"],
        signatures: &[],
        related_formats: &[],
    },
};
