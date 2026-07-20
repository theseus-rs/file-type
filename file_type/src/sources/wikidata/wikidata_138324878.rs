use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324878: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_878,
        source_type: SourceType::Wikidata,
        name: "PROV-N Provenance Notation",
        extensions: &["provn"],
        media_types: &["text/provenance-notation"],
        signatures: &[],
        related_formats: &[],
    },
};
