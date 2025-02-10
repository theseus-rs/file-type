use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_74550562: FileType = FileType {
    file_format: &FileFormat {
        id: 74_550_562,
        source_type: SourceType::Wikidata,
        name: "SAP2000 DataBase",
        extensions: &["sdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
