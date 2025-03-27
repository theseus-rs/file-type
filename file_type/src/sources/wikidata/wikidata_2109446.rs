use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2109446: FileType = FileType {
    file_format: &FileFormat {
        id: 2_109_446,
        source_type: SourceType::Wikidata,
        name: "Dialogic ADPCM",
        extensions: &["vox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
