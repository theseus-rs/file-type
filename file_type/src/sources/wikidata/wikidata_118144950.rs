use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118144950: FileType = FileType {
    file_format: &FileFormat {
        id: 118_144_950,
        source_type: SourceType::Wikidata,
        name: "Serenade Symphony File",
        extensions: &["sph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
