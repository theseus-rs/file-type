use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113674482: FileType = FileType {
    file_format: &FileFormat {
        id: 113_674_482,
        source_type: SourceType::Wikidata,
        name: "3D Landscape 2.0 File",
        extensions: &["3sl", "lnd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
