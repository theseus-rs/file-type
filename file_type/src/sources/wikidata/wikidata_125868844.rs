use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125868844: FileType = FileType {
    file_format: &FileFormat {
        id: 125_868_844,
        source_type: SourceType::Wikidata,
        name: "Common Data Format dotCDF 2.6-2.7",
        extensions: &["cdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
