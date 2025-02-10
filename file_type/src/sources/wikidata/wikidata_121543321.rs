use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121543321: FileType = FileType {
    file_format: &FileFormat {
        id: 121_543_321,
        source_type: SourceType::Wikidata,
        name: "TaxCut 2008 Tax Return File",
        extensions: &["t08"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
