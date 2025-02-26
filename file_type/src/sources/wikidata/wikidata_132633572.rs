use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132633572: FileType = FileType {
    file_format: &FileFormat {
        id: 132_633_572,
        source_type: SourceType::Wikidata,
        name: "Modula-3 M2SDS data file format",
        extensions: &["rfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
