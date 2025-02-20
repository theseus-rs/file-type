use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50809751: FileType = FileType {
    file_format: &FileFormat {
        id: 50_809_751,
        source_type: SourceType::Wikidata,
        name: "Portable Database, version 1",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
