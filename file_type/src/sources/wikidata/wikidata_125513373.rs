use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125513373: FileType = FileType {
    file_format: &FileFormat {
        id: 125_513_373,
        source_type: SourceType::Wikidata,
        name: "Sinar RAW Image",
        extensions: &["cs1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
