use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117485972: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_972,
        source_type: SourceType::Wikidata,
        name: "Audacity Project File 3.x",
        extensions: &["aup3"],
        media_types: &["application/x-audacity-project+sqlite3"],
        signatures: &[],
        related_formats: &[],
    },
};
