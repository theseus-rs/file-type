use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27960157: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_157,
        source_type: SourceType::Wikidata,
        name: "Matroska Audio",
        extensions: &["mka"],
        media_types: &["audio/matroska", "audio/x-matroska"],
        signatures: &[],
        related_formats: &[],
    },
};
