use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_15671948: FileType = FileType {
    file_format: &FileFormat {
        id: 15_671_948,
        source_type: SourceType::Wikidata,
        name: "Blend file",
        extensions: &["blend"],
        media_types: &["application/x-blender"],
        signatures: &[],
        related_formats: &[],
    },
};
