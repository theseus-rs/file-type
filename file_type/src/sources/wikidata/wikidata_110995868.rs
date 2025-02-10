use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110995868: FileType = FileType {
    file_format: &FileFormat {
        id: 110_995_868,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Half-fold Card File format",
        extensions: &["hcr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
