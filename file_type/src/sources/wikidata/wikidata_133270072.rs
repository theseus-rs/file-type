use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133270072: FileType = FileType {
    file_format: &FileFormat {
        id: 133_270_072,
        source_type: SourceType::Wikidata,
        name: "GeoConcept text file",
        extensions: &["gxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
