use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333329: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_329,
        source_type: SourceType::Wikidata,
        name: "PSION A-law file",
        extensions: &["psion"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
