use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95985515: FileType = FileType {
    file_format: &FileFormat {
        id: 95_985_515,
        source_type: SourceType::Wikidata,
        name: "Affymetrix PSI file format",
        extensions: &["psi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
