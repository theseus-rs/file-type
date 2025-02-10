use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342124: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_124,
        source_type: SourceType::Wikidata,
        name: "Sonic Foundry sample resource file",
        extensions: &["sfr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
