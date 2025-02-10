use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111333316: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_316,
        source_type: SourceType::Wikidata,
        name: "WAVmaker program file",
        extensions: &["prg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
