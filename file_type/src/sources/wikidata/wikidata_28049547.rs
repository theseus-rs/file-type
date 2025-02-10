use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28049547: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_547,
        source_type: SourceType::Wikidata,
        name: "STAD image",
        extensions: &["pac", "seq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
