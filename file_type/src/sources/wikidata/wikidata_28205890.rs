use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205890: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_890,
        source_type: SourceType::Wikidata,
        name: "Multipage Z-Soft Paintbrush Bitmap Graphics",
        extensions: &["dcx"],
        media_types: &["image/x-dcx"],
        signatures: &[],
        related_formats: &[],
    },
};
