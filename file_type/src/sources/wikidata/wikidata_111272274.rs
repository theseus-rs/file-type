use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272274: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_274,
        source_type: SourceType::Wikidata,
        name: "Ensoniq KT disk image",
        extensions: &["edk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
