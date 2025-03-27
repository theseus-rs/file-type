use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133455024: FileType = FileType {
    file_format: &FileFormat {
        id: 133_455_024,
        source_type: SourceType::Wikidata,
        name: "Amiga Metafile",
        extensions: &["amf"],
        media_types: &["image/x-amff"],
        signatures: &[],
        related_formats: &[],
    },
};
