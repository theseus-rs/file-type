use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51913488: FileType = FileType {
    file_format: &FileFormat {
        id: 51_913_488,
        source_type: SourceType::Wikidata,
        name: "Fractal Design Painter RIFF Bitmap Graphics",
        extensions: &["rif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
