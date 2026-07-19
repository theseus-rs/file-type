use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324880: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_880,
        source_type: SourceType::Wikidata,
        name: "RIF XML",
        extensions: &["rif"],
        media_types: &["application/rif+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
