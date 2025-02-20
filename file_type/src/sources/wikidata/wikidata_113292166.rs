use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113292166: FileType = FileType {
    file_format: &FileFormat {
        id: 113_292_166,
        source_type: SourceType::Wikidata,
        name: "Macintosh Sound Resource",
        extensions: &["snd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
