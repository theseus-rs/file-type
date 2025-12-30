use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136457589: FileType = FileType {
    file_format: &FileFormat {
        id: 136_457_589,
        source_type: SourceType::Wikidata,
        name: "XYZ Point Cloud file format",
        extensions: &["xyz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
