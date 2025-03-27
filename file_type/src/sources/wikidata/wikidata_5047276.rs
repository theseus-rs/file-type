use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5047276: FileType = FileType {
    file_format: &FileFormat {
        id: 5_047_276,
        source_type: SourceType::Wikidata,
        name: "Cartesian Perceptual Compression",
        extensions: &["cpc", "cpi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
