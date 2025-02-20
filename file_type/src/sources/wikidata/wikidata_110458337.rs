use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110458337: FileType = FileType {
    file_format: &FileFormat {
        id: 110_458_337,
        source_type: SourceType::Wikidata,
        name: "SpritePad Image Format",
        extensions: &["spd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
