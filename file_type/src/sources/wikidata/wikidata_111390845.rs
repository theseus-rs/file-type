use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111390845: FileType = FileType {
    file_format: &FileFormat {
        id: 111_390_845,
        source_type: SourceType::Wikidata,
        name: "Bryce Object File",
        extensions: &["obp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
