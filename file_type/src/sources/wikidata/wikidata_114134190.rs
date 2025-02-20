use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114134190: FileType = FileType {
    file_format: &FileFormat {
        id: 114_134_190,
        source_type: SourceType::Wikidata,
        name: "MOPAC dataset format",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
