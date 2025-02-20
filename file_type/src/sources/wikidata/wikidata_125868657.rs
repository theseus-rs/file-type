use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125868657: FileType = FileType {
    file_format: &FileFormat {
        id: 125_868_657,
        source_type: SourceType::Wikidata,
        name: "Common Data Format dotCDF 2.0-2.5",
        extensions: &["cdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
