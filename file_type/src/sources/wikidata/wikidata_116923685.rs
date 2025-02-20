use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116923685: FileType = FileType {
    file_format: &FileFormat {
        id: 116_923_685,
        source_type: SourceType::Wikidata,
        name: "Super Duper Music Looper File",
        extensions: &["sdml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
