use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_108837049: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_049,
        source_type: SourceType::Wikidata,
        name: "Nero CD-ROM (Hybrid) Compilation",
        extensions: &["nrh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
