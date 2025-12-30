use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136714229: FileType = FileType {
    file_format: &FileFormat {
        id: 136_714_229,
        source_type: SourceType::Wikidata,
        name: "Enhanced Metafile Format Plus Extensions",
        extensions: &["emf", "emz"],
        media_types: &["image/emf"],
        signatures: &[],
        related_formats: &[],
    },
};
