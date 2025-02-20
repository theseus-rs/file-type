use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120717288: FileType = FileType {
    file_format: &FileFormat {
        id: 120_717_288,
        source_type: SourceType::Wikidata,
        name: "TaxCut 2007 Tax Return file",
        extensions: &["t07"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
