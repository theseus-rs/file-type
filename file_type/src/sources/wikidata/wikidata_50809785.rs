use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50809785: FileType = FileType {
    file_format: &FileFormat {
        id: 50_809_785,
        source_type: SourceType::Wikidata,
        name: "Portable Database, version 3",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
