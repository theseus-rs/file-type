use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_108837022: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_022,
        source_type: SourceType::Wikidata,
        name: "Nero Mixed Mode CD Compilation",
        extensions: &["nrm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
