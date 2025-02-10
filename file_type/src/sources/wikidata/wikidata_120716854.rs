use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120716854: FileType = FileType {
    file_format: &FileFormat {
        id: 120_716_854,
        source_type: SourceType::Wikidata,
        name: "TaxCut 2006 Tax Return file",
        extensions: &["t06"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
