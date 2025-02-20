use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125868958: FileType = FileType {
    file_format: &FileFormat {
        id: 125_868_958,
        source_type: SourceType::Wikidata,
        name: "Common Data Format dotCDF 3.x",
        extensions: &["cdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
