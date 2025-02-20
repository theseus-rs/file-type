use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167432: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_432,
        source_type: SourceType::Wikidata,
        name: "NuFX",
        extensions: &["bxy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
