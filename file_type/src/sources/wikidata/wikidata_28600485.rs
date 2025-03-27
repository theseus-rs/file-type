use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600485: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_485,
        source_type: SourceType::Wikidata,
        name: "DWC",
        extensions: &["dwc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
