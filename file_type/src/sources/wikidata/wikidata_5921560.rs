use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5921560: FileType = FileType {
    file_format: &FileFormat {
        id: 5_921_560,
        source_type: SourceType::Wikidata,
        name: "Synthetic music mobile application format",
        extensions: &["m3f", "mmf", "mqf"],
        media_types: &["application/vnd.smaf"],
        signatures: &[],
        related_formats: &[],
    },
};
