use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5531823: FileType = FileType {
    file_format: &FileFormat {
        id: 5_531_823,
        source_type: SourceType::Wikidata,
        name: "General Data Format for Biomedical Signals",
        extensions: &["gdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
