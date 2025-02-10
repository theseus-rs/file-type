use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979381: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_381,
        source_type: SourceType::Wikidata,
        name: "Blu-ray Clip info",
        extensions: &["clp", "clpi", "cpi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
