use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
