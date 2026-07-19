use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138746673: FileType = FileType {
    file_format: &FileFormat {
        id: 138_746_673,
        source_type: SourceType::Wikidata,
        name: "Fusion 360 Archive  file",
        extensions: &["fsch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
