use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71264369: FileType = FileType {
    file_format: &FileFormat {
        id: 71_264_369,
        source_type: SourceType::Wikidata,
        name: "Hippel COmpressed SOng module",
        extensions: &["hipc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
