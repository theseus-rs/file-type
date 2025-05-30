use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049547: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_547,
        source_type: SourceType::Wikidata,
        name: "STAD image",
        extensions: &["pac", "seq"],
        media_types: &["image/x-stad"],
        signatures: &[],
        related_formats: &[],
    },
};
