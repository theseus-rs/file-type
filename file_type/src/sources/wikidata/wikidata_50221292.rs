use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50221292: FileType = FileType {
    file_format: &FileFormat {
        id: 50_221_292,
        source_type: SourceType::Wikidata,
        name: "JEOL NMR Spectroscopy",
        extensions: &["jdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
