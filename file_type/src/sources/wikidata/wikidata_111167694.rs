use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111167694: FileType = FileType {
    file_format: &FileFormat {
        id: 111_167_694,
        source_type: SourceType::Wikidata,
        name: "ChemBasic file",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
