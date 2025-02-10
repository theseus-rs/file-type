use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113291185: FileType = FileType {
    file_format: &FileFormat {
        id: 113_291_185,
        source_type: SourceType::Wikidata,
        name: "Serif Metafile Format",
        extensions: &["smf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
