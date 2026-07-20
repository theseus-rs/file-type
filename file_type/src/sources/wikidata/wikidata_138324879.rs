use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324879: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_879,
        source_type: SourceType::Wikidata,
        name: "PROV-XML Provenance Notation",
        extensions: &["provx"],
        media_types: &["application/provenance+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
